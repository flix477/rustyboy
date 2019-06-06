use crate::processor::instruction::InstructionInfo;
use crate::processor::registers::Registers;

pub struct DebugInfo<'a> {
    pub registers: &'a Registers,
    pub line: u16,
    pub instruction: &'a InstructionInfo,
}