use crate::debugger::breakpoint::Breakpoint;
use crate::debugger::commands::breakpoint::BreakpointAction;
use crate::debugger::debug_info::{ProcessorDebugInfo, DebugInfo};

pub mod breakpoint;
pub mod commands;
pub mod debug_info;
pub mod debug_operand_parser;

#[derive(Clone)]
pub struct Debugger {
    pub breakpoints: Vec<Breakpoint>,
    pub forced_break: bool,
}

impl Default for Debugger {
    fn default() -> Self {
        Self {
            breakpoints: Vec::new(),
            forced_break: false,
        }
    }
}

impl Debugger {
    pub fn should_run(&self, debug_info: &ProcessorDebugInfo) -> bool {
        self.forced_break
            || self
                .breakpoints
                .iter()
                .any(|breakpoint| breakpoint.satisfied(debug_info))
    }

    pub fn run_action(&mut self, action: DebuggerAction<'_>) -> DebuggerActionResult {
        if self.forced_break {
            self.forced_break = false;
        };

        match action {
            DebuggerAction::Breakpoint(action) => commands::breakpoint::run(action, self),
            DebuggerAction::Continue => DebuggerActionResult::Resume,
            DebuggerAction::StepInto => commands::step_into::run(self),
            DebuggerAction::StepOver(debug_info) => commands::step_over::run(self, debug_info)
        }
    }

    pub fn clean_breakpoints(&mut self, debug_info: &ProcessorDebugInfo) {
        self.breakpoints = self
            .breakpoints
            .iter()
            .filter(|breakpoint| !breakpoint.one_time || !breakpoint.satisfied(debug_info))
            .cloned()
            .collect()
    }
}

#[derive(Clone)]
pub enum DebuggerAction<'a> {
    Breakpoint(BreakpointAction),
    Continue,
    StepInto,
    StepOver(&'a DebugInfo)
}

pub enum DebuggerActionResult {
    Resume,
    None,
}

#[cfg(test)]
mod tests {
    use crate::debugger::breakpoint::Breakpoint;
    use crate::debugger::commands::breakpoint::BreakpointAction;
    use crate::debugger::{Debugger, DebuggerAction};

    #[test]
    fn adds_breakpoint() {
        let mut debugger = Debugger::default();

        let breakpoint = Breakpoint {
            conditions: vec![],
            one_time: false,
        };

        debugger.run_action(DebuggerAction::Breakpoint(BreakpointAction::Add(
            breakpoint,
        )));

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

        debugger.run_action(DebuggerAction::Breakpoint(BreakpointAction::Remove(0)));

        assert_eq!(debugger.breakpoints.len(), 1);
    }
}
