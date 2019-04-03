use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult};
use crate::debugger::{DebugInfo, DebuggerState};

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
        _: &[&str],
        debugger: &mut DebuggerState,
        _: &DebugInfo,
        _: &Bus,
    ) -> CommandResult {
        debugger.forced_break = true;
        CommandResult::Quit
    }
}
