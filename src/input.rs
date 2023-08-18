const CONCAT_CHARACTER: &str = "+";
const MAXIMUM_REPEAT: usize = 20;

use std::{future::IntoFuture, collections::VecDeque, sync::{Arc, Mutex}};

use crate::{gamepad::Inputs, service, stats::Stats};

pub async fn collect_inputs_queue(
    inputs_queue: &mut VecDeque<Inputs<bool>>,
    services: &[Box<dyn service::Service>],
    stats: Arc<Mutex<Stats>>,
) -> bool {
    let mut updated = false;

    for service in services.iter().map(|service| service.as_ref()) {
        let service = service.get_inputs().into_future().await;
        if let Ok(mut inputs) = service.lock() {
            let mut stats = stats.lock().unwrap();

            inputs.iter().for_each(|input| {                
                stats.all_messages += 1;

                let mut inputs_vector = parse_input(input);

                if !inputs_vector.is_empty() {
                    stats.input_messages += 1;
                    stats.inputs += inputs_vector.len() as u128;
                    updated = true;
                    inputs_queue.append(&mut inputs_vector);
                }
            });
            inputs.clear();
        } else {
            // error
        };
    }
    updated
}

macro_rules! inputs_set {
    ($buttons:ident; $input:expr => $output:expr) => {
        match $input {
            "a" => $buttons.a = $output,
            "b" => $buttons.b = $output,
            "x" => $buttons.x = $output,
            "y" => $buttons.y = $output,
            "up"|"u" => $buttons.up = $output,
            "left"|"l" => $buttons.left = $output,
            "down"|"d" => $buttons.down = $output,
            "right"|"r" => $buttons.right = $output,
            "start"|"s" => $buttons.start = $output,
            "select" => $buttons.select = $output,
            _ => {}
        }
    };
}

pub fn parse_input(string: &str) -> VecDeque<Inputs<bool>> {
    let is_digit = char::is_ascii_digit;

    string
        .split(char::is_whitespace)
        .filter_map(|slice| {
            let repeat = slice
                .chars()
                .take_while(is_digit)
                .collect::<String>()
                .parse::<usize>()
                .unwrap_or(1)
                .clamp(1, MAXIMUM_REPEAT);

            let slice = slice.trim_start_matches(|c| is_digit(&c));

            let mut inputs = Inputs::default();
            let count = slice.split(CONCAT_CHARACTER)
                .map(|t| {
                    inputs_set!(inputs; t.to_lowercase().as_str() => true);
                })
                .count();
            if count > 0 && inputs != Inputs::default() {
                Some(vec![inputs; repeat])
            } else {
                None
            }
        })
        .flatten()
        .collect()
}
