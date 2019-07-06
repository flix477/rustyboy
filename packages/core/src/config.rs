use crate::debugger::Debugger;
use crate::gameboy::DeviceType;

pub struct Config {
    pub device_type: DeviceType,
    pub debugger: Option<Debugger>,
}
