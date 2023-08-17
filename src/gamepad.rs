const GAMEPAD_NAME: &str = "gamepad";

pub const PRESS_TIME: Duration = Duration::from_millis(6 * 16);
pub const RELEASE_TIME: Duration = Duration::from_millis(5 * 16);
pub const MAX_UPDATE_TIME: Duration = Duration::from_millis(500);

use std::{time::{Instant, Duration}, collections::VecDeque};
use vigem_client::{ XGamepad, Xbox360Wired, XButtons, Client, TargetId };
use async_trait::async_trait;

use crate::service::{debug, DebugType, Abortable};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Inputs<T> {
    pub a: T,
    pub b: T,
    pub x: T,
    pub y: T,
    pub up: T,
    pub left: T,
    pub down: T,
    pub right: T,
    pub start: T,
    pub select: T,
}

impl Inputs<Instant> {
    pub fn new() -> Self {
        Inputs {
            a: Instant::now(),
            b: Instant::now(),
            x: Instant::now(),
            y: Instant::now(),
            up: Instant::now(),
            left: Instant::now(),
            down: Instant::now(),
            right: Instant::now(),
            start: Instant::now(),
            select: Instant::now(),
        }
    } 
}

pub struct Gamepad {
    client: Xbox360Wired<Client>,
    input: XGamepad,
}

impl Gamepad {
    pub fn new() -> Self {
        let client = Client::connect().unwrap();
        let id = TargetId::XBOX360_WIRED;
        let mut target = Xbox360Wired::new(client, id);
    
        target.plugin().unwrap();
        target.wait_ready().unwrap();
    
        debug(
            GAMEPAD_NAME.to_string(),
            DebugType::Ready,
        );
    
        Gamepad {
            client: target,
            input: XGamepad::default(),
        }
    }
    
    
    pub fn update_gamepad(
        &mut self,
        inputs_queue: &mut VecDeque<Inputs<bool>>,
        inputs: &mut Inputs<Instant>,
        last_update: &mut Instant,
        release_timer: &mut Instant,
    ) {
        let pressed_buttons =
            if inputs.a.elapsed() < PRESS_TIME { XButtons!(A).into() } else { 0 } |
            if inputs.b.elapsed() < PRESS_TIME { XButtons!(B).into() } else { 0 } |
            if inputs.x.elapsed() < PRESS_TIME { XButtons!(X).into() } else { 0 } |
            if inputs.y.elapsed() < PRESS_TIME { XButtons!(Y).into() } else { 0 } |
            if inputs.up.elapsed() < PRESS_TIME { XButtons!(UP).into() } else { 0 } |
            if inputs.left.elapsed() < PRESS_TIME { XButtons!(LEFT).into() } else { 0 } |
            if inputs.down.elapsed() < PRESS_TIME { XButtons!(DOWN).into() } else { 0 } |
            if inputs.right.elapsed() < PRESS_TIME { XButtons!(RIGHT).into() } else { 0 } |
            if inputs.start.elapsed() < PRESS_TIME { XButtons!(START).into() } else { 0 } |
            if inputs.select.elapsed() < PRESS_TIME { XButtons!(BACK).into() } else { 0 };
    
        if pressed_buttons == 0 {
            if release_timer.elapsed() > RELEASE_TIME {
                if let Some(new_input) = inputs_queue.pop_front() {
                    if new_input.a { inputs.a = Instant::now() };
                    if new_input.b { inputs.b = Instant::now() };
                    if new_input.x { inputs.x = Instant::now() };
                    if new_input.y { inputs.y = Instant::now() };
                    if new_input.up { inputs.up = Instant::now() };
                    if new_input.left { inputs.left = Instant::now() };
                    if new_input.down { inputs.down = Instant::now() };
                    if new_input.right { inputs.right = Instant::now() };
                    if new_input.start { inputs.start = Instant::now() };
                    if new_input.select { inputs.select = Instant::now() };
                }
            }
        } else {
            *release_timer = Instant::now();
        }
    
        let previous_inputs = self.input;
        self.input.buttons.raw = pressed_buttons;
    
        if previous_inputs != self.input || last_update.elapsed() > MAX_UPDATE_TIME {
            self.client.update(&self.input).unwrap();
            *last_update = Instant::now();

            // // this gets spammed too often
            // debug(
            //     GAMEPAD_NAME.to_string(),
            //     DebugType::Ping,
            // );
        }
    }
    
}

impl Default for Gamepad {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Abortable for Gamepad {
    async fn abort(&mut self) {
        self.client.unplug().unwrap();
        debug(
            GAMEPAD_NAME.to_string(),
            DebugType::Abort,
        );
    }
}

