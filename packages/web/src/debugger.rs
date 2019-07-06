use wasm_bindgen::prelude::*;

use rustyboy_core::debugger::breakpoint::{Breakpoint, BreakpointCondition};
use rustyboy_core::debugger::commands::breakpoint::BreakpointAction;
use rustyboy_core::debugger::{Debugger, DebuggerAction, DebuggerActionResult};
use rustyboy_core::processor::registers::RegisterType;

#[wasm_bindgen(js_name = Debugger)]
pub struct DebuggerJs {
    #[wasm_bindgen(skip)]
    pub debugger: Debugger,
}
//
#[wasm_bindgen(js_class = Debugger)]
impl DebuggerJs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DebuggerJs {
        DebuggerJs {
            debugger: Debugger::default()
        }
    }

    #[wasm_bindgen(js_name = addBreakpoint)]
    pub fn add_breakpoint(&mut self, condition: BreakpointConditionJs) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(self.debugger.run_action(DebuggerAction::Breakpoint(
            BreakpointAction::Add(Breakpoint {
                conditions: vec![condition.into()],
                one_time: false,
            }),
        )))
    }

    #[wasm_bindgen(js_name = removeBreakpoint)]
    pub fn remove_breakpoint(&mut self, index: usize) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(
            self.debugger
                .run_action(DebuggerAction::Breakpoint(BreakpointAction::Remove(index))),
        )
    }
    //
    #[wasm_bindgen(js_name = stepInto)]
    pub fn step_into(&mut self) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(self.debugger.run_action(DebuggerAction::StepInto))
    }

    #[wasm_bindgen(js_name = continueExecution)]
    pub fn continue_execution(&mut self) -> DebuggerActionResultJs {
        DebuggerActionResultJs::from(self.debugger.run_action(DebuggerAction::Continue))
    }
}

#[wasm_bindgen(js_name = BreakpointCondition)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct BreakpointConditionJs {
    pub register: RegisterTypeJs,
    pub value: u16,
}

impl Into<BreakpointCondition> for BreakpointConditionJs {
    fn into(self) -> BreakpointCondition {
        BreakpointCondition::RegisterEquals(self.register.into(), self.value)
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
