use super::{Command, CommandResult, DebuggerState};
use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;

const MATCHING_VALUES: &[&str] = &["stepinto", "si"];

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
        _: &DebugInfo<'_>,
        _: &dyn Bus,
    ) -> CommandResult {
        debugger.forced_break = true;
        CommandResult::Quit
    }
}
