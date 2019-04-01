use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult};
use crate::debugger::{DebugInfo, DebuggerState};

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
