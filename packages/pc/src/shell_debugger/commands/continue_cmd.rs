use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;
use super::{Command, CommandResult, DebuggerState};

const MATCHING_VALUES: &'static [&'static str] = &["continue", "c"];

pub struct ContinueCommand {}

impl ContinueCommand {
    pub fn create_command() -> Box<dyn Command> {
        Box::new(ContinueCommand {})
    }
}

impl Command for ContinueCommand {
    fn matching_value(&self) -> &[&str] {
        MATCHING_VALUES
    }

    fn execute(&self, _: &[&str], _: &mut DebuggerState, _: &DebugInfo, _: &Bus) -> CommandResult {
        CommandResult::Quit
    }
}
