#![feature(never_type)]
const PROJECT_NAME: &str = "placogaviki";

mod stats;
mod cli;
mod input;
mod gamepad;
mod service;
mod dotenv;

use service::{Service, DebugType, debug, Abortable};
use gamepad::{Inputs, Gamepad};
use stats::Stats;

use std::process::exit;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use input::collect_inputs_queue;
use serenity::FutureExt;
use serenity::futures::future::join_all;

#[tokio::main]
async fn main() {
    let mut services: Vec<Box<dyn service::Service>> = vec![
        Box::new(service::kick::Kick::new().await),
        Box::new(service::discord::Discord::new().await),
        Box::new(service::youtube::Youtube::new().await),
    ];

    let stats = Arc::new(Mutex::new(Stats::default()));

    let mut inputs_queue = VecDeque::new();

    let mut gamepad = Gamepad::default();

    let mut last_update = Instant::now();
    let mut release_timer = Instant::now();

    let mut inputs = Inputs::new();

    debug(
        PROJECT_NAME.to_string(),
        DebugType::Ready,
    );

    let mut aborted = false;

    let mut cli_output = cli::setup(stats.clone()).boxed();

    loop {
        sleep(Duration::from_millis(10));
        
        collect_inputs_queue(&mut inputs_queue, &services, stats.clone()).await;

        gamepad.update_gamepad(
            &mut inputs_queue,
            &mut inputs,
            &mut last_update,
            &mut release_timer
        );

        if (&mut cli_output).now_or_never().is_some() { break }
    }

    abort_services(&mut services, &mut gamepad, &mut aborted).await;

    debug(
        PROJECT_NAME.to_string(),
        DebugType::Abort,
    );

    exit(0);
}

pub async fn abort_services(
    services: &mut [Box<dyn service::Service>],
    gamepad: &mut Gamepad,
    aborted: &mut bool
) {
    if *aborted { return }

    gamepad.abort().await;
    join_all(
        services
            .iter_mut()
            .map(|service| service.abort())
    ).await;
    
    *aborted = true;
}
