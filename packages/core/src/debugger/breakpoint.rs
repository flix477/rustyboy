use crate::debugger::debug_info::ProcessorDebugInfo;
use crate::processor::registers::RegisterType;

#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoint {
    pub conditions: Vec<BreakpointCondition>,
    pub one_time: bool,
}

impl Breakpoint {
    pub fn satisfied(&self, debug_info: &ProcessorDebugInfo) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.satisfied(debug_info))
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum BreakpointCondition {
    RegisterEquals(RegisterType, u16),
}

impl BreakpointCondition {
    pub fn satisfied(self, debug_info: &ProcessorDebugInfo) -> bool {
        match self {
            BreakpointCondition::RegisterEquals(register, value) => {
                debug_info.registers.reg(register) == value
            }
        }
    }
}
