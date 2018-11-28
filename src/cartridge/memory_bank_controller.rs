use cartridge::cartridge_capability::CartridgeCapability;
use cartridge::cartridge_metadata::CartridgeMetadata;
use bus::Bus;
use util::bits::get_bit;
use std::cmp;

pub struct MBCFactory {}
impl MBCFactory {
    pub fn from_metadata(metadata: &CartridgeMetadata)
        -> Option<Box<dyn MemoryBankController>>
    {
        let variant = MBCVariant::from_capabilities(&metadata.capabilities)?;
        let ram_size = metadata.rom_size;
        let rom_size = metadata.ram_size;
        match variant {
            MBCVariant::MBC1 => Some(Box::new(MBC1::new())),
            _ => None
        }
    }
}

pub trait MemoryBankController {
    fn rom_bank(&self) -> u8;
    fn ram_bank(&self) -> u8;
    fn ram_enabled(&self) -> bool;
    fn write_rom(&mut self, address: u16, value: u8) {}

    fn relative_rom_address(&self, address: u16) -> usize {
        let current_bank = self.rom_bank() as usize;
        address as usize + current_bank * 0xFF
    }

    fn relative_ram_address(&self, address: u16) -> usize {
        let current_bank = self.ram_bank() as usize;
//        address as usize + current_bank * 0xFF
        0
    }
}

struct MBC1 {
    mode: MBC1Mode,
    rom_bank: u8,
    ram_enabled: bool,
    ram_bank: u8
}

impl MBC1 {
    pub fn new() -> MBC1 {
        MBC1 {
            mode: MBC1Mode::MaxROM,
            rom_bank: 1,
            ram_enabled: false,
            ram_bank: 1
        }
    }
}

impl MemoryBankController for MBC1 {
    fn rom_bank(&self) -> u8 { self.rom_bank }

    fn ram_bank(&self) -> u8 { self.ram_bank }

    fn ram_enabled(&self) -> bool { self.ram_enabled }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            2000...0x3FFF => { // change rom bank
                self.rom_bank = cmp::max(value & 0b11111, 1);
            },
            6000...0x7FFF => { // change mode
                self.mode = if get_bit(value, 7) {
                    MBC1Mode::MaxRAM
                } else {
                    MBC1Mode::MaxROM
                };
            },
            _ => {}
        }

        if let MBC1Mode::MaxRAM = self.mode {
            match address {
                0...0x1FFF => { // toggle ram bank
                    self.ram_enabled = value == 0x0A;
                },
                4000...0x5FFF => { // change ram bank
                    self.ram_bank = cmp::max(value & 0b11, 1);
                },
                _ => {}
            }
        } else {

        }
    }
}

pub enum MBC1Mode {
    MaxROM,
    MaxRAM
}

pub enum MBCVariant {
    MBC1,
    MBC2,
    MBC3,
    MBC4,
    MBC5
}

impl MBCVariant {
    pub fn from_capabilities(capabilities: &Vec<CartridgeCapability>)
        -> Option<MBCVariant>
    {
        if capabilities.contains(&CartridgeCapability::MBC1) {
            Some(MBCVariant::MBC1)
        } else if capabilities.contains(&CartridgeCapability::MBC2) {
            Some(MBCVariant::MBC2)
        } else if capabilities.contains(&CartridgeCapability::MBC3) {
            Some(MBCVariant::MBC3)
        } else if capabilities.contains(&CartridgeCapability::MBC4) {
            Some(MBCVariant::MBC4)
        } else if capabilities.contains(&CartridgeCapability::MBC5) {
            Some(MBCVariant::MBC5)
        } else {
            None
        }
    }
}