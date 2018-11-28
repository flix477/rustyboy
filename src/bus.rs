use cartridge::Cartridge;
use std::error::Error;

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct Hardware {
    cartridge: Cartridge
}

impl Hardware {
    pub fn new() -> Result<Hardware, Box<dyn Error>> {
        Ok(Hardware {
            cartridge: Cartridge::from_file("")?
        })
    }
}

impl Bus for Hardware {
    fn read(&self, address: u16) -> u8 {
        match address {
            0...0x7FFF |
            0xA000...0xBFFF => self.cartridge.read(address),
            0x8000...0x9FFF => 0, // video ram,
            0xC000...0xCFFF => 0, // 8kb internal ram
            0xD000...0xDFFF => 0, // echo ^^
            0xFE00...0xFEFF => 0, // sprite attrib
            0xFF00...0xFF7F => 0, // i/o ports
            0xFF80...0xFFFE => 0, // internal ram
            0xFFFF => 0, // interrupt enable
            _ => 0 // empty
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0...0x7FFF |
            0xA000...0xBFFF => self.cartridge.write(address, value),
            0x8000...0x9FFF => {}, // video ram,
            0xC000...0xCFFF => {}, // 8kb internal ram
            0xD000...0xDFFF => {}, // echo ^^
            0xFE00...0xFEFF => {}, // sprite attrib
            0xFF00...0xFF7F => {}, // i/o ports
            0xFF80...0xFFFE => {}, // internal ram
            0xFFFF => {}, // interrupt enable
            _ => {} // empty
        }
    }
}