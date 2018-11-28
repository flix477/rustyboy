pub mod cartridge_metadata;
mod cartridge_capability;
mod memory_bank_controller;

use std::fs;
use std::error::Error;
use cartridge::cartridge_metadata::CartridgeMetadata;
use bus::Bus;
use cartridge::memory_bank_controller::{MemoryBankController, MBCFactory};

pub struct Cartridge {
    buffer: Vec<u8>,
    metadata: CartridgeMetadata,
    mbc: Option<Box<dyn MemoryBankController>>
}

impl Cartridge {
    pub fn from_file(filename: &str) -> Result<Cartridge, Box<dyn Error>> {
        return Cartridge::from_buffer(fs::read(filename)?);
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<Cartridge, Box<dyn Error>> {
        let metadata = CartridgeMetadata::from_buffer(&buffer)?;
        let mbc = MBCFactory::from_metadata(&metadata);
        return Ok(Cartridge {
            metadata,
            buffer,
            mbc
        });
    }
}

impl Bus for Cartridge {
    fn read(&self, address: u16) -> u8 {
        match address {
            0...0x3FFF => self.buffer[address as usize], // first rom bank
            0x4000...0x7FFF => {
                let address = if let Some(mbc) = &self.mbc {
                    mbc.relative_rom_address(address)
                } else { address as usize };
                self.buffer[address]
            }, // switchable rom bank
            0xA000...0xBFFF => 0, // switchable ram bank
            _ => 0
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0...0x7FFF => {
                if let Some(ref mut mbc) = self.mbc {
                    mbc.write_rom(address, value);
                }
            },
            0xA000...0xBFFF => {}, // switchable ram bank
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_pokemon_blue() {
        assert!(Cartridge::from_file("pokemonb.gb").is_ok());
    }
}