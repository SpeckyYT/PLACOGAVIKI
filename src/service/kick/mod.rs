mod message;
mod chatrooms;

use super::{Service, DebugType, debug, Abortable};
use message::v2::*;
use chatrooms::CHATROOM_IDS;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use json::object::Object;
use json::{JsonValue, object};
use websocket::{OwnedMessage, ClientBuilder};
use async_trait::async_trait;



const SERVICE_NAME: &str = "kick";
// const PUSHER_CLUSTER: &str = "us2";
// const PUSHER_KEY: &str = "eb1d5f283081a78b932c";
const WEBSOCKET_URL: &str = "ws://ws-us2.pusher.com/app/eb1d5f283081a78b932c?protocol=7&client=js&version=7.4.0&flash=false";

pub struct Kick {
    inputs: Arc<Mutex<VecDeque<String>>>,
    join_task: JoinHandle<()>,
    task: JoinHandle<!>,
}

#[async_trait]
impl Service for Kick {
    async fn new() -> Self {
        let inputs = Arc::new(Mutex::new(VecDeque::new()));
        let inputs_intern = inputs.clone();

        let (read, write, join_task) = chat_setup().await;

        debug(
            SERVICE_NAME.to_string(),
            DebugType::Ready,
        );

        let task = tokio::task::spawn(async move {
            loop {
                let message = read.lock().unwrap().recv_message(); 
                match message {
                    Ok(OwnedMessage::Text(string)) => {
                        if let Ok(json) = json::parse(&string) {
                            let data =
                                if let Some(Ok(JsonValue::Object(data))) = json["data"].as_str().map(json::parse) {
                                    data
                                } else if let JsonValue::Object(data) = &json["data"] {
                                    data.clone()
                                } else {
                                    debug(
                                        SERVICE_NAME.to_string(),
                                        DebugType::Custom {
                                            title: "INVALID JSON".to_string(),
                                            extra: "".to_string(),
                                        },
                                    );
                                    Object::new()
                                };
                            
                            if data["type"].as_str().unwrap_or_default() != "message" { continue; }

                            let message = Message::new(&data);
                            debug(
                                SERVICE_NAME.to_string(),
                                DebugType::Message(message.content.clone()),
                            );
                            let mut inputs = inputs_intern.lock().unwrap();
                            inputs.push_back(message.content);
                        }
                    },
                    Ok(OwnedMessage::Ping(content)) => {
                        debug(
                            SERVICE_NAME.to_string(),
                            DebugType::Ping,
                        );
                        write.lock().unwrap().send_message(&websocket::Message::pong(content)).unwrap();
                    },
                    Ok(_) => debug(
                        SERVICE_NAME.to_string(),
                        DebugType::Custom {
                            title: "OTHER MESSAGE".to_string(),
                            extra: "".to_string(),
                        },
                    ),
                    Err(err) => debug(
                        SERVICE_NAME.to_string(),
                        DebugType::Custom {
                            title: "ERROR".to_string(),
                            extra: format!("{:?}", err),
                        },
                    ),
                }
            }
        });

        Kick {
            inputs,
            task,
            join_task,
        }
    }
    async fn get_inputs(&self) -> Arc<Mutex<VecDeque<String>>> {
        self.inputs.clone()
    }
}

#[async_trait]
impl Abortable for Kick {
    async fn abort(&mut self) {
        self.task.abort();
        self.join_task.abort();
        debug(
            SERVICE_NAME.to_string(),
            DebugType::Abort,
        );
    }
}

async fn chat_setup() -> (
    Arc<Mutex<websocket::sync::Reader<std::net::TcpStream>>>,
    Arc<Mutex<websocket::sync::Writer<std::net::TcpStream>>>,
    JoinHandle<()>,
) {
    let (read_chat, write_chat) = ClientBuilder::new(WEBSOCKET_URL)
        .unwrap()
        .connect_insecure()
        .unwrap()
        .split()
        .unwrap();

    let read_chat = Arc::new(Mutex::new(read_chat));
    let write_chat = Arc::new(Mutex::new(write_chat));

    let write_chat_internal = write_chat.clone();

    let join_task = tokio::task::spawn(async move {
        for chatroom_id in CHATROOM_IDS {
            debug(
                SERVICE_NAME.to_string(),
                DebugType::Custom {
                    title: "JOIN ROOM".to_string(),
                    extra: chatroom_id.to_string(),
                },
            );
            write_chat_internal.lock().unwrap().send_message(&websocket::Message::text(object! {
                event: "pusher:subscribe",
                data: {
                    auth: "",
                    channel: format!("chatrooms.{}.v2", chatroom_id),
                },
            }.to_string())).unwrap();
        }
    });

    (read_chat, write_chat, join_task)
}
