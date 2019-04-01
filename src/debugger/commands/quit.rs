use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult};
use crate::debugger::{DebugInfo, DebuggerState};

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

    fn execute(
        &self,
        _: &[&str],
        _: &mut DebuggerState,
        _: &DebugInfo,
        _: &Bus,
    ) -> CommandResult {
        std::process::exit(0);
    }
}
