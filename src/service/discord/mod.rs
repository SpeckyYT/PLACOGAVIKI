const SERVICE_NAME: &str = "discord";

use super::{Service, DebugType, debug, Abortable};
use crate::dotenv::DOTENV;
use std::{collections::VecDeque, sync::{Mutex, Arc}};

use async_trait::async_trait;
use serenity::{prelude::*, FutureExt};
use serenity::model::{channel::Message, gateway::Ready};

pub struct Discord {
    client: Client,
    inputs: Arc<Mutex<VecDeque<String>>>,
}

pub struct Handler {
    inputs: Arc<Mutex<VecDeque<String>>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.bot { return }
        if message.content.is_empty() { return }
        let inputs = self.inputs.clone();
        let mut inputs = inputs.lock().unwrap();
        debug(
            SERVICE_NAME.to_string(),
            DebugType::Message(message.content.clone()),
        );
        inputs.push_back(message.content);
    }
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        debug(
            SERVICE_NAME.to_string(),
            DebugType::Ready,
        );
    }
}

#[async_trait]
impl Service for Discord {
    async fn new() -> Self {
        let inputs = Arc::new(Mutex::new(VecDeque::new()));

        let mut discord = Discord {
            client: {
                let handler = Handler { inputs: inputs.clone() };
                Client::builder(
                    &DOTENV.discord_token,
                    GatewayIntents:: MESSAGE_CONTENT
                    | GatewayIntents::GUILD_MESSAGES
                    | GatewayIntents::DIRECT_MESSAGES
                ).event_handler(handler).await.unwrap()
            },
            inputs: inputs.clone(),
        };

        assert!(discord.client.start().now_or_never().is_none());

        discord
    }
    async fn get_inputs(&self) -> Arc<Mutex<VecDeque<String>>> {
        self.inputs.clone()
    }
}

#[async_trait]
impl Abortable for Discord {
    async fn abort(&mut self) {
        self.client.shard_manager.lock().await.shutdown_all().await; // stop client
        debug(
            SERVICE_NAME.to_string(),
            DebugType::Abort,
        );
    }
}
