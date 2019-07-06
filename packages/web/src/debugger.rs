use wasm_bindgen::prelude::*;

use rustyboy_core::debugger::{Debugger, DebuggerAction};
use rustyboy_core::debugger::commands::breakpoint::BreakpointAction;
use rustyboy_core::debugger::breakpoint::{BreakpointCondition, Breakpoint};
use rustyboy_core::processor::registers::RegisterType;

#[wasm_bindgen(js_name = Debugger)]
pub struct DebuggerJs {
    #[wasm_bindgen(skip)]
    debugger: Debugger,
}
//
//#[wasm_bindgen(js_class = Debugger)]
//impl DebuggerJs {
//    #[wasm_bindgen(js_name = runAction)]
//    pub fn add_breakpoint(&mut self, conditions: Vec<BreakpointConditionJs>) {
//        self.debugger.run_action(
//            DebuggerAction::Breakpoint(
//                BreakpointAction::Add(Breakpoint {
//                    conditions: conditions.iter().map(|x| x.into()).collect(),
//                    one_time: false
//                })
//            ),
//
//        );
//    }
//}

#[wasm_bindgen(js_name = BreakpointCondition)]
pub struct BreakpointConditionJs {
    pub register: RegisterTypeJs,
    pub value: u16
}

impl Into<BreakpointCondition> for BreakpointConditionJs {
    fn into(self) -> BreakpointCondition {
        BreakpointCondition::RegisterEquals(self.register.into(), self.value)
    }
}

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