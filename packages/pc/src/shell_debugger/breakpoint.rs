use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::processor::registers::RegisterType;

#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoint {
    pub line: u16,
    pub conditions: Option<Vec<BreakpointCondition>>,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum BreakpointCondition {
    RegisterEquals(RegisterType, u16),
}

impl BreakpointCondition {
    pub fn satisfied(self, debug_info: &DebugInfo<'_>) -> bool {
        match self {
            BreakpointCondition::RegisterEquals(register, value) => {
                debug_info.registers.reg(register) == value
            }
        }
    }
}
