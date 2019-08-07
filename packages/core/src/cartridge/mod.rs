mod cartridge_capability;
pub mod cartridge_metadata;
mod mbc;

use crate::bus::{Readable, Writable};
use crate::cartridge::cartridge_metadata::CartridgeMetadata;
use crate::cartridge::mbc::{MBCFactory, MemoryBankController};
use std::error::Error;
use std::fs;

pub struct Cartridge {
    buffer: Vec<u8>,
    metadata: CartridgeMetadata,
    mbc: Option<Box<dyn MemoryBankController>>,
}

impl Cartridge {
    pub fn from_file(filename: &str) -> Result<Cartridge, Box<dyn Error>> {
        Cartridge::from_buffer(fs::read(filename)?)
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<Cartridge, Box<dyn Error>> {
        let metadata = CartridgeMetadata::from_buffer(&buffer)?;
        let mbc = MBCFactory::from_metadata(&metadata);
        Ok(Cartridge {
            metadata,
            buffer,
            mbc,
        })
    }

    pub fn metadata(&self) -> &CartridgeMetadata {
        &self.metadata
    }

    pub fn reset(&mut self) {
        self.mbc = MBCFactory::from_metadata(&self.metadata);
    }
}

impl Readable for Cartridge {
    fn read(&self, address: u16) -> u8 {
        match address {
            0..=0x3FFF => self.buffer[address as usize], // first rom bank
            0x4000..=0x7FFF => {
                let address = if let Some(mbc) = &self.mbc {
                    mbc.relative_rom_address(address as usize)
                } else {
                    address as usize
                };
                self.buffer[address]
            } // switchable rom bank
            0xA000..=0xBFFF => {
                if let Some(mbc) = &self.mbc {
                    if mbc.ram_enabled() {
                        return mbc.read_ram(address as usize, &self.buffer);
                    }
                }
                0 // TODO: should do something else maybe?
            } // switchable ram bank
            _ => 0,
        }
    }
}

impl Writable for Cartridge {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0..=0x7FFF => {
                if let Some(ref mut mbc) = self.mbc {
                    mbc.write_rom(address as usize, value);
                }
            }
            0xA000..=0xBFFF => {
                if let Some(ref mut mbc) = self.mbc {
                    if mbc.ram_enabled() {
                        mbc.write_ram(address as usize, value, &mut self.buffer);
                    }
                }
            } // switchable ram bank
            _ => {}
        }
    }
}
