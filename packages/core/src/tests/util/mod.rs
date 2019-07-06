use crate::debugger::debug_info::{DebugInfo, ProcessorDebugInfo};
use crate::processor::registers::Registers;

pub mod mock_bus;

pub fn mock_debug_info(registers: Registers, bus: Vec<u8>) -> DebugInfo {
    let cpu_debug_info = ProcessorDebugInfo { registers };

    DebugInfo {
        cpu_debug_info,
        bus,
    }
}
