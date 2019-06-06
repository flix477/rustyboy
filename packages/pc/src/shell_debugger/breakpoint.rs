use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::processor::instruction::Mnemonic;
use rustyboy_core::processor::registers::RegisterType;

#[derive(Clone)]
pub struct Breakpoint {
    pub line: u16,
    pub conditions: Option<Vec<BreakpointCondition>>,
}

#[derive(Copy, Clone)]
pub enum BreakpointCondition {
    RegisterEquals(RegisterType, u16),
    MnemonicEquals(Mnemonic),
}

impl BreakpointCondition {
    pub fn satisfied(&self, debug_info: &DebugInfo) -> bool {
        match self {
            BreakpointCondition::RegisterEquals(register, value) => {
                debug_info.registers.reg(*register) == *value
            }
            BreakpointCondition::MnemonicEquals(mnemonic) => {
                debug_info.instruction.mnemonic() == mnemonic
            }
        }
    }
}
