use crate::debugger::Debugger;
use crate::gameboy::DeviceType;

pub struct Config {
    pub device_type: DeviceType,
    pub debugger: Option<Debugger>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_type: DeviceType::GameBoy,
            debugger: None
        }
    }
}
