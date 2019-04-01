use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult, Debugger};
use crate::debugger::{DebugInfo, DebuggerState};
use crate::processor::registers::Registers;

const MATCHING_VALUES: &'static [&'static str] = &["stepinto", "si"];

pub struct StepIntoCommand {}

impl StepIntoCommand {
    pub fn create_command() -> Box<dyn Command> {
        Box::new(StepIntoCommand {})
    }
}

impl Command for StepIntoCommand {
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
        debugger.forced_break = true;
        CommandResult::Continue
    }
}
