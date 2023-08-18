pub mod channels;

use super::{Service, Abortable, debug, DebugType};
use channels::{LiveSource, LIVE_SOURCES};
use std::{sync::{Arc,Mutex}, collections::VecDeque, time::Duration};
use youtube_chat::{live_chat::LiveChatClientBuilder, item::MessageItem};
use tokio::{task, task::JoinHandle, time};
use async_trait::async_trait;

const SERVICE_NAME: &str = "youtube";
const FETCH_DELAY: u64 = 500; // in milliseconds

pub struct Youtube {
    inputs: Arc<Mutex<VecDeque<String>>>,
    tasks: Vec<JoinHandle<()>>,
}

#[async_trait]
impl Abortable for Youtube {
    async fn abort(&mut self) {
        self.tasks.iter().map(|task| task.abort()).for_each(drop);
        debug(
            SERVICE_NAME.to_string(),
            DebugType::Abort,
        );
    }
}

#[async_trait]
impl Service for Youtube {
    async fn new() -> Self {
        let inputs = Arc::new(Mutex::new(VecDeque::new()));

        let mut tasks = vec![];

        for live_source in LIVE_SOURCES {
            let inputs = inputs.clone();

            let task = task::spawn(async move {
                let debug = |data| {
                    debug(
                        SERVICE_NAME.to_string(),
                        data,
                    );
                };


                let dirty_id = match live_source {
                    LiveSource::Channel(s) => s,
                    LiveSource::Live(s) => s,
                };

                let client = LiveChatClientBuilder::new()
                .on_start(|video_id|
                    debug(DebugType::Custom {
                        title: "join live".to_string(),
                        extra: video_id,
                    })
                )
                .on_chat(|c| {
                    let message_content: String = c.message.iter().map(|v| match v {
                        MessageItem::Text(text) => text.clone(),
                        MessageItem::Emoji(emoji) => emoji.emoji_text.clone().unwrap(),
                    }).collect();
    
                    debug(DebugType::Message(message_content.clone()));
    
                    let mut inputs = inputs.lock().unwrap();
                    inputs.push_back(message_content);
                })
                .on_end(||
                    debug(DebugType::Custom {
                        title: "stream end".to_string(),
                        extra: dirty_id.to_string(),
                    })
                )
                .on_error(|e| debug(DebugType::Error(format!("{:?}", e))));

                let mut client = match live_source {
                    LiveSource::Channel(id) => client.channel_id(id.to_string()),
                    LiveSource::Live(id) => client.live_id(id.to_string()),
                }.build();

                match client.start().await {
                    Ok(_) => {
                        debug(DebugType::Ready);
                        let mut interval = time::interval(Duration::from_millis(FETCH_DELAY));
                        loop {
                            interval.tick().await;
                            client.execute().await;
                        };
                    },
                    Err(error) => {
                        debug(DebugType::Error(format!("{:?}", error)));
                    }
                }
            });
            tasks.push(task);
        }

        Youtube {
            inputs,
            tasks,
        }
    }
    async fn get_inputs(&self) -> Arc<Mutex<VecDeque<String>>> {
        self.inputs.clone()
    }
}
