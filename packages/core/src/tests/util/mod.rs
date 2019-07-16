use crate::debugger::debug_info::ProcessorDebugInfo;
use crate::processor::registers::Registers;

pub mod mock_bus;

pub fn mock_debug_info<'a>(registers: Registers, bus: Vec<u8>) -> ProcessorDebugInfo {
    ProcessorDebugInfo {
        registers,
        bus,
    }
}
