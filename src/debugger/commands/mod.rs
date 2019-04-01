use crate::bus::Bus;
use crate::debugger::{DebugInfo, Debugger, DebuggerState};
use crate::processor::registers::Registers;

pub mod breakpoint;
pub mod status;
pub mod step_into;
pub mod step_over;

pub enum CommandResult {
    Continue,
    Quit,
    None,
}

pub trait Command {
    fn matching_value(&self) -> &[&str];
    fn execute(
        &self,
        input: &[&str],
        debugger: &mut DebuggerState,
        debug_info: &DebugInfo,
        bus: &Bus,
    ) -> CommandResult;
}
