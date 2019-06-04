use crate::cartridge::Cartridge;
use crate::debugger::DebuggerState;
use crate::gameboy::DeviceType;

pub struct Config {
    pub cartridge: Cartridge,
    pub device_type: DeviceType,
    pub debugger_config: Option<DebuggerState>,
}
