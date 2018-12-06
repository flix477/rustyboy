pub mod cartridge_metadata;
mod cartridge_capability;
mod mbc;

use std::fs;
use std::error::Error;
use cartridge::cartridge_metadata::CartridgeMetadata;
use bus::{Readable, Writable};
use cartridge::mbc::{MemoryBankController, MBCFactory};

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

impl Readable for Cartridge {
    fn read(&self, address: u16) -> u8 {
        match address {
            0...0x3FFF => self.buffer[address as usize], // first rom bank
            0x4000...0x7FFF => {
                let address = if let Some(mbc) = &self.mbc {
                    mbc.relative_rom_address(address as usize)
                } else { address as usize };
                self.buffer[address]
            }, // switchable rom bank
            0xA000...0xBFFF => {
                if let Some(mbc) = &self.mbc {
                    if mbc.ram_enabled() {
                        let address = address as usize + self.metadata.rom_size;
                        return mbc.read_ram(address, &self.buffer);
                    }
                }
                return 0; // TODO: should do something else maybe?
            }, // switchable ram bank
            _ => 0
        }
    }
}

impl Writable for Cartridge {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0...0x7FFF => {
                if let Some(ref mut mbc) = self.mbc {
                    mbc.write_rom(address as usize, value);
                }
            },
            0xA000...0xBFFF => {
                if let Some(ref mut mbc) = self.mbc {
                    if mbc.ram_enabled() {
                        mbc.write_ram(address as usize, value, &mut self.buffer);
                    }
                }
            }, // switchable ram bank
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