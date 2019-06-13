use super::{Command, CommandResult, DebuggerState};
use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;

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
        _: &mut DebuggerState,
        _: &DebugInfo<'_>,
        _: &dyn Bus,
    ) -> CommandResult {
        CommandResult::Quit
    }
}
