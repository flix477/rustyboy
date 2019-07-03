use crate::debugger::breakpoint::Breakpoint;
use crate::debugger::{Debugger, DebuggerActionResult};

#[derive(Clone, PartialEq, Debug)]
pub enum BreakpointAction {
    Add(Breakpoint),
    Remove(usize),
}

pub fn run(action: BreakpointAction, debugger: &mut Debugger) -> DebuggerActionResult {
    match action {
        BreakpointAction::Add(breakpoint) => debugger.breakpoints.push(breakpoint),
        BreakpointAction::Remove(index) => {
            debugger.breakpoints = debugger
                .breakpoints
                .iter()
                .enumerate()
                .filter(|(idx, _)| index != *idx)
                .map(|(_, breakpoint)| breakpoint)
                .cloned()
                .collect();
        }
    }

    DebuggerActionResult::None
}
