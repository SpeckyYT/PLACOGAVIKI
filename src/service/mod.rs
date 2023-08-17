use std::{sync::{Arc, Mutex}, collections::VecDeque};
use async_trait::async_trait;
use colored::Colorize;

pub mod discord;
pub mod kick;

#[derive(Debug, Clone)]
pub enum DebugType {
    Ready,
    Ping,
    Abort,
    Message(String),
    Error(String),
    Custom {
        title: String,
        extra: String,
    },
}

#[async_trait]
pub trait Abortable {
    async fn abort(&mut self);
}

#[async_trait]
pub trait Service: Abortable {
    async fn new() -> Self where Self: Sized;
    async fn get_inputs(&self) -> Arc<Mutex<VecDeque<String>>>;
}

pub fn debug(service_name: String, debug: DebugType) {
    use DebugType::*;

    let service = format!("[{}]", service_name.to_uppercase());
    
    let category = format!("[{}]", match &debug {
        Ready => "READY".to_string(),
        Ping => "PING".to_string(),
        Abort => "ABORT".to_string(),
        Message(_) => "MESSAGE".to_string(),
        Error(_) => "ERROR".to_string(),
        Custom { title, .. } => title.to_uppercase(),
    });

    

    let extra = match &debug {
        Ready | Ping | Abort => None,
        Message(message) => Some(message),
        Error(error) => Some(error),
        Custom { extra, .. } => Some(extra),
    }.map(|m| format!("{:#?}", m)).unwrap_or("".to_string());

    println!("{} {} {}", service.cyan(), category.green(), extra.blue());
}
