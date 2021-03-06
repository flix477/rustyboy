use wasm_bindgen::prelude::*;

use crate::debugger::debug_info::DebugInfoJs;
use rustyboy_core::debugger::breakpoint::{Breakpoint, BreakpointCondition};
use rustyboy_core::debugger::commands::breakpoint::BreakpointAction;
use rustyboy_core::debugger::{Debugger, DebuggerAction, DebuggerActionResult};
use rustyboy_core::processor::registers::RegisterType;

pub mod debug_info;

#[wasm_bindgen(js_name = Debugger)]
pub struct DebuggerJs {
    #[wasm_bindgen(skip)]
    pub debugger: Debugger,
}

impl Default for DebuggerJs {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_class = Debugger)]
impl DebuggerJs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DebuggerJs {
        DebuggerJs {
            debugger: Debugger::default(),
        }
    }

    #[wasm_bindgen(js_name = addBreakpoint)]
    pub fn add_breakpoint(
        &mut self,
        register: RegisterTypeJs,
        value: u16,
        one_time: bool,
    ) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(self.debugger.run_action(DebuggerAction::Breakpoint(
            BreakpointAction::Add(Breakpoint {
                conditions: vec![BreakpointCondition::RegisterEquals(register.into(), value)],
                one_time,
            }),
        )))
    }

    #[wasm_bindgen]
    pub fn breakpoints(&self) -> Vec<u16> {
        self.debugger
            .breakpoints
            .iter()
            .filter_map(|breakpoint| {
                breakpoint
                    .conditions
                    .iter()
                    .filter_map(|condition| {
                        if let BreakpointCondition::RegisterEquals(RegisterType::PC, value) =
                            condition
                        {
                            Some(value)
                        } else {
                            None
                        }
                    })
                    .next()
                    .copied()
            })
            .collect()
    }

    #[wasm_bindgen(js_name = removeBreakpoint)]
    pub fn remove_breakpoint(&mut self, index: usize) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(
            self.debugger
                .run_action(DebuggerAction::Breakpoint(BreakpointAction::Remove(index))),
        )
    }

    #[wasm_bindgen(js_name = stepInto)]
    pub fn step_into(&mut self) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(self.debugger.run_action(DebuggerAction::StepInto))
    }

    #[wasm_bindgen(js_name = stepOver)]
    pub fn step_over(&mut self, debug_info: &DebugInfoJs) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(
            self.debugger
                .run_action(DebuggerAction::StepOver(&debug_info.debug_info)),
        )
    }

    #[wasm_bindgen(js_name = continueExecution)]
    pub fn continue_execution(&mut self) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(self.debugger.run_action(DebuggerAction::Continue))
    }
}

// This is quite ugly, finding a way to make this copy the original enum would be great
#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegisterTypeJs {
    AF,
    A,
    F,
    BC,
    B,
    C,
    DE,
    D,
    E,
    HL,
    H,
    L,
    SP,
    PC,
}

impl Into<RegisterType> for RegisterTypeJs {
    fn into(self) -> RegisterType {
        match self {
            RegisterTypeJs::AF => RegisterType::AF,
            RegisterTypeJs::A => RegisterType::A,
            RegisterTypeJs::F => RegisterType::F,
            RegisterTypeJs::BC => RegisterType::BC,
            RegisterTypeJs::B => RegisterType::B,
            RegisterTypeJs::C => RegisterType::C,
            RegisterTypeJs::DE => RegisterType::DE,
            RegisterTypeJs::D => RegisterType::D,
            RegisterTypeJs::E => RegisterType::E,
            RegisterTypeJs::HL => RegisterType::HL,
            RegisterTypeJs::H => RegisterType::H,
            RegisterTypeJs::L => RegisterType::L,
            RegisterTypeJs::SP => RegisterType::SP,
            RegisterTypeJs::PC => RegisterType::PC,
        }
    }
}

#[wasm_bindgen(js_name = DebuggerActionResult)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DebuggerActionResultJs {
    Resume,
    None,
}

impl From<DebuggerActionResult> for DebuggerActionResultJs {
    fn from(value: DebuggerActionResult) -> DebuggerActionResultJs {
        match value {
            DebuggerActionResult::Resume => DebuggerActionResultJs::Resume,
            DebuggerActionResult::None => DebuggerActionResultJs::None,
        }
    }
}
