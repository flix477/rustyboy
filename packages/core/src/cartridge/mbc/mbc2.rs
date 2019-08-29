use super::MemoryBankController;
use crate::cartridge::cartridge_capability::CartridgeCapability;
use std::cmp;
use crate::util::savestate::{read_savestate_bool, read_savestate_byte, LoadSavestateError, Savestate};

// TODO: RAM is 512*4bit, maybe should only return the 4 necessary bits on read
pub struct MBC2 {
    rom_bank: u8,
    ram_enabled: bool,
}

impl MBC2 {
    pub fn new(_capabilities: &[CartridgeCapability]) -> MBC2 {
        MBC2 {
            rom_bank: 1,
            ram_enabled: false,
        }
    }
}

impl Savestate for MBC2 {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.rom_bank);
        buffer.push(self.ram_enabled as u8);
    }

    fn load_savestate<'a>(&mut self, buffer: &mut std::slice::Iter<u8>) -> Result<(), LoadSavestateError> {
        self.rom_bank = read_savestate_byte(buffer)?;
        self.ram_enabled = read_savestate_bool(buffer)?;
        Ok(())
    }
}

impl MemoryBankController for MBC2 {
    fn rom_bank(&self) -> u16 {
        u16::from(self.rom_bank)
    }

    fn ram_bank(&self) -> u8 {
        0
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_rom(&mut self, address: usize, value: u8) {
        match address {
            0..=0x1FFF => {
                // toggle ram bank
                if address & 0x100 == 0 {
                    self.ram_enabled = value == 0x0A;
                }
            }
            0x2000..=0x3FFF => {
                // change rom bank
                if address & 0x100 != 0 {
                    self.rom_bank = cmp::max(value & 0xF, 1);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_bank_switching() {
        let mut mbc = MBC2::new(&[]);

        // shouldn't work
        mbc.write_rom(0x2000, 15);
        assert_ne!(mbc.rom_bank(), 15);

        // should work
        mbc.write_rom(0x2100, 15);
        assert_eq!(mbc.rom_bank(), 15);
    }

    #[test]
    fn rom_bank_switching_zero() {
        let mut mbc = MBC2::new(&[]);
        mbc.write_rom(0x2100, 0);
        assert_eq!(mbc.rom_bank(), 1);
    }

    #[test]
    fn enable_ram() {
        let mut mbc = MBC2::new(&[]);

        // shouldn't work, bad address
        mbc.write_rom(0x0100, 0x0A);
        assert!(!mbc.ram_enabled());

        // shouldn't work, bad value
        mbc.write_rom(0x0000, 0x0B);
        assert!(!mbc.ram_enabled());

        // should work
        mbc.write_rom(0x0000, 0x0A);
        assert!(mbc.ram_enabled());
    }
}
