use crate::debugger::{Debugger, DebuggerActionResult};

pub fn run(debugger: &mut Debugger) -> DebuggerActionResult {
    debugger.forced_break = true;
    DebuggerActionResult::Resume
}
