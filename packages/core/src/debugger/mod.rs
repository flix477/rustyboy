use crate::debugger::breakpoint::Breakpoint;
use crate::debugger::commands::breakpoint::BreakpointAction;
use crate::debugger::debug_info::ProcessorDebugInfo;

pub mod breakpoint;
pub mod commands;
pub mod debug_info;

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
            || self.breakpoints.iter().any(|breakpoint| {
                breakpoint
                    .conditions
                    .iter()
                    .all(|condition| condition.satisfied(debug_info))
            })
    }

    pub fn run_action(&mut self, action: DebuggerAction) -> DebuggerActionResult {
        if self.forced_break {
            self.forced_break = false;
        };
        match action {
            DebuggerAction::Breakpoint(action) => commands::breakpoint::run(action, self),
            DebuggerAction::Continue => DebuggerActionResult::Resume,
            DebuggerAction::StepInto => commands::step_into::run(self),
        }
    }
}

#[derive(Clone)]
pub enum DebuggerAction {
    Breakpoint(BreakpointAction),
    Continue,
    StepInto,
}

pub enum DebuggerActionResult {
    Resume,
    None,
}
