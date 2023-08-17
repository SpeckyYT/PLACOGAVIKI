use std::sync::{Arc, Mutex};
use async_std::io::stdin;

use crate::stats::Stats;
use crate::service::{debug, DebugType};
use crate::dotenv::DOTENV;

pub fn setup(stats: Arc<Mutex<Stats>>) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async move {
        let stdin = stdin();

        loop {
            let mut line = String::new();
            stdin.read_line(&mut line).await.unwrap();
            let line = line.trim();

            let debug = |debug_data| debug("cli".to_string(), debug_data);

            match line {
                "stats" => {
                    match stats.lock() {
                        Ok(stats) => stats.print(),
                        Err(error) =>
                            debug(DebugType::Error(format!("{:?}", error))),
                    }
                },
                "env" => {
                    DOTENV.print("cli".to_string());
                },
                "exit"|"quit" => break,
                _ => debug(DebugType::Error("Invalid input".to_string())),
            }
        }
    })
}
