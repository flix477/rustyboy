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

#[cfg(test)]
mod tests {
    use crate::debugger::breakpoint::Breakpoint;
    use crate::debugger::commands::breakpoint::{run, BreakpointAction};
    use crate::debugger::Debugger;

    #[test]
    fn adds_breakpoint() {
        let mut debugger = Debugger::default();

        let breakpoint = Breakpoint {
            conditions: vec![],
            one_time: false,
        };

        run(BreakpointAction::Add(breakpoint), &mut debugger);

        assert_eq!(debugger.breakpoints.len(), 1);
    }

    #[test]
    fn removes_breakpoint() {
        let mut debugger = Debugger {
            breakpoints: vec![
                Breakpoint {
                    conditions: vec![],
                    one_time: false,
                },
                Breakpoint {
                    conditions: vec![],
                    one_time: false,
                },
            ],
            forced_break: false,
        };

        run(BreakpointAction::Remove(0), &mut debugger);

        assert_eq!(debugger.breakpoints.len(), 1);
    }

    #[test]
    fn remove_breakpoint_bad_index() {
        let mut debugger = Debugger {
            breakpoints: vec![Breakpoint {
                conditions: vec![],
                one_time: false,
            }],
            forced_break: false,
        };

        run(BreakpointAction::Remove(1), &mut debugger);

        assert_eq!(debugger.breakpoints.len(), 1);
    }
}
