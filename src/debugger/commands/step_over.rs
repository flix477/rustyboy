use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult, Debugger};
use crate::debugger::{DebugInfo, DebuggerState};
use crate::processor::registers::Registers;

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

    fn execute(
        &self,
        input: &[&str],
        debugger: &mut DebuggerState,
        debug_info: &DebugInfo,
        _: &Bus,
    ) -> CommandResult {
        // this is stupid, will refactor later
        debugger.breakpoints.insert(debug_info.line + 1);
        CommandResult::Quit
    }
}
