use crate::service::{debug, DebugType};

#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub all_messages: u128,
    pub input_messages: u128,
    pub inputs: u128,
}

impl Stats {
    pub fn print(&self) {
        let debug = |title, extra|
            debug(
                "cli".to_string(),
                DebugType::Custom {
                    title,
                    extra,
                }
            );

        debug("all messages".to_string(), self.all_messages.to_string());
        debug("input messages".to_string(), self.input_messages.to_string());
        debug("inputs".to_string(), self.inputs.to_string());
    }
}
