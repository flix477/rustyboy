use crate::cartridge::Cartridge;
use crate::gameboy::DeviceType;

pub struct Config {
    pub cartridge: Cartridge,
    pub device_type: DeviceType
}
