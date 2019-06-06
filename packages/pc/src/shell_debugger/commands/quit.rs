use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;
use super::{Command, CommandResult, DebuggerState};

const MATCHING_VALUES: &'static [&'static str] = &["quit", "q"];

pub struct QuitCommand {}

impl QuitCommand {
    pub fn create_command() -> Box<dyn Command> {
        Box::new(QuitCommand {})
    }
}

impl Command for QuitCommand {
    fn matching_value(&self) -> &[&str] {
        MATCHING_VALUES
    }

    fn execute(&self, _: &[&str], _: &mut DebuggerState, _: &DebugInfo, _: &Bus) -> CommandResult {
        std::process::exit(0);
    }
}
