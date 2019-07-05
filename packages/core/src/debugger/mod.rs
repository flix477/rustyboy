use crate::debugger::breakpoint::Breakpoint;
use crate::debugger::commands::breakpoint::BreakpointAction;
use crate::debugger::debug_info::{DebugInfo, ProcessorDebugInfo};

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

    pub fn run_action(
        &mut self,
        action: DebuggerAction,
        debug_info: &DebugInfo,
    ) -> DebuggerActionResult {
        if self.forced_break {
            self.forced_break = false;
        };

        self.clean_breakpoints(&debug_info.cpu_debug_info);

        match action {
            DebuggerAction::Breakpoint(action) => commands::breakpoint::run(action, self),
            DebuggerAction::Continue => DebuggerActionResult::Resume,
            DebuggerAction::StepInto => commands::step_into::run(self),
        }
    }

    pub fn clean_breakpoints(&mut self, debug_info: &ProcessorDebugInfo) {
        self.breakpoints = self
            .breakpoints
            .iter()
            .filter(|breakpoint| !breakpoint.satisfied(debug_info))
            .cloned()
            .collect()
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
