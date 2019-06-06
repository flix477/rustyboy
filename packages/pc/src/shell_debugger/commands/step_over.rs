use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;
use super::{Command, CommandResult, DebuggerState};

const MATCHING_VALUES: &'static [&'static str] = &["stepover", "so"];

pub struct StepOverCommand {}

impl StepOverCommand {
    pub fn create_command() -> Box<dyn Command> {
        Box::new(StepOverCommand {})
    }
}

impl Command for StepOverCommand {
    fn matching_value(&self) -> &[&str] {
        MATCHING_VALUES
    }

    fn execute(&self, _: &[&str], _: &mut DebuggerState, _: &DebugInfo, _: &Bus) -> CommandResult {
        // this is stupid, will refactor later
        //        debugger.breakpoints.insert(debug_info.line + 1);
        CommandResult::Quit
    }
}
