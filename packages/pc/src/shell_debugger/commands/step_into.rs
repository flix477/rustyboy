use super::{Command, CommandResult, Debugger};
use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::debugger::DebuggerAction;

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
        debugger: &mut Debugger,
        _: &DebugInfo,
    ) -> CommandResult {
        CommandResult::from(debugger.run_action(DebuggerAction::StepInto))
    }
}
