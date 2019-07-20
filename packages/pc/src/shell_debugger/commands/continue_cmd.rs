use super::{Command, CommandResult, Debugger};
use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::debugger::DebuggerAction;

const MATCHING_VALUES: &[&str] = &["continue", "c"];

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
        _: &[&str],
        debugger: &mut Debugger,
        _: &DebugInfo,
    ) -> CommandResult {
        CommandResult::from(debugger.run_action(DebuggerAction::Continue))
    }
}
