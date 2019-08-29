mod cartridge_capability;
pub mod cartridge_metadata;
mod mbc;

use crate::bus::{Readable, Writable};
use crate::cartridge::cartridge_metadata::CartridgeMetadata;
use crate::cartridge::mbc::{MBCFactory, MemoryBankController};
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::util::savestate::{Savestate, LoadSavestateError};

pub struct Cartridge {
    buffer: Vec<u8>,
    metadata: CartridgeMetadata,
    mbc: Option<Box<dyn MemoryBankController>>,
    pub ram: Option<Vec<u8>>,
}

impl Cartridge {
    pub fn from_file(filename: &str) -> Result<Cartridge, Box<dyn Error>> {
        let mut cartridge = Cartridge::from_buffer(fs::read(filename)?)?;

        if cartridge.ram.is_some() {
            let ram_path = Path::new(filename).with_extension("sav");
            if let Ok(ram_buffer) = fs::read(ram_path) {
                cartridge.ram = Some(ram_buffer);
            }
        }

        Ok(cartridge)
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<Cartridge, Box<dyn Error>> {
        let metadata = CartridgeMetadata::from_buffer(&buffer)?;
        let mbc = MBCFactory::from_metadata(&metadata);

        let ram = if metadata.ram_size > 0 {
            Some(vec![0; metadata.ram_size])
        } else {
            None
        };

        Ok(Cartridge {
            metadata,
            buffer,
            mbc,
            ram,
        })
    }

    pub fn metadata(&self) -> &CartridgeMetadata {
        &self.metadata
    }

    pub fn reset(&mut self) {
        self.mbc = MBCFactory::from_metadata(&self.metadata);
    }

    fn read_ram(&self, address: usize) -> u8 {
        if let (Some(mbc), Some(ram)) = (&self.mbc, &self.ram) {
            if mbc.ram_enabled() {
                return mbc.read_ram(address as usize, ram);
            }
        }

        0xFF
    }

    fn write_ram(&mut self, address: usize, value: u8) {
        if let (Some(mbc), Some(ram)) = (&self.mbc, &mut self.ram) {
            if mbc.ram_enabled() {
                mbc.write_ram(address as usize, value, ram);
            }
        }
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
            0xA000..=0xBFFF => self.read_ram(address as usize), // switchable ram bank
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
            0xA000..=0xBFFF => self.write_ram(address as usize, value), // switchable ram bank
            _ => {}
        }
    }
}

impl Savestate for Cartridge {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        if let Some(mbc) = &self.mbc {
            mbc.dump_savestate(buffer);
        }

        if let Some(ram) = &self.ram {
            buffer.append(&mut ram.clone());
        }
    }

    fn load_savestate<'a>(&mut self, buffer: &mut std::slice::Iter<u8>) -> Result<(), LoadSavestateError> {
        if let Some(ref mut mbc) = self.mbc {
            mbc.load_savestate(buffer)?;
        }

//        if let Some(ref mut ram) = self.ram {
//            std::mem::replace(ram, buffer.take(ram.len()).cloned().collect());
//        }

        Ok(())
    }
}