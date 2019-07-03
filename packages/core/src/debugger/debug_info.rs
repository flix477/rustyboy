use crate::processor::instruction::InstructionInfo;
use crate::processor::registers::Registers;

pub struct ProcessorDebugInfo {
    pub registers: Registers,
    pub instruction: InstructionInfo,
}

pub struct DebugInfo {
    pub cpu_debug_info: ProcessorDebugInfo,
    pub bus: Vec<u8>,
}
