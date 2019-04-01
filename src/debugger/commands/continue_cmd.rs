use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult, Debugger};
use crate::debugger::{DebugInfo, DebuggerState};
use crate::processor::registers::Registers;

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

    fn execute(
        &self,
        input: &[&str],
        debugger: &mut DebuggerState,
        debug_info: &DebugInfo,
        _: &Bus,
    ) -> CommandResult {
        CommandResult::Quit
    }
}
