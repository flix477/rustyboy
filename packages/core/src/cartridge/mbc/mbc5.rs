use super::MemoryBankController;
use crate::cartridge::cartridge_capability::CartridgeCapability;
use crate::util::savestate::{Savestate, LoadSavestateError, read_savestate_byte, read_savestate_bool, write_savestate_u16};

pub struct MBC5 {
    rom_bank: u16,
    ram_enabled: bool,
    ram_bank: u8,
}

impl MBC5 {
    pub fn new(_capabilities: &[CartridgeCapability]) -> MBC5 {
        MBC5 {
            rom_bank: 0,
            ram_enabled: false,
            ram_bank: 0,
        }
    }

    pub fn set_ram_enabled(&mut self, enabled: bool) {
        self.ram_enabled = enabled;
    }
}

impl Savestate for MBC5 {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        write_savestate_u16(buffer, self.rom_bank);
        buffer.push(self.ram_enabled as u8);
        buffer.push(self.ram_bank);
    }

    fn load_savestate<'a>(&mut self, buffer: &mut std::slice::Iter<u8>) -> Result<(), LoadSavestateError> {
        self.rom_bank = u16::from(read_savestate_byte(buffer)?);
        self.rom_bank |= u16::from(read_savestate_byte(buffer)?) << 8;
        self.ram_enabled = read_savestate_bool(buffer)?;
        self.ram_bank = read_savestate_byte(buffer)?;
        Ok(())
    }
}

impl MemoryBankController for MBC5 {
    fn rom_bank(&self) -> u16 {
        self.rom_bank
    }

    fn ram_bank(&self) -> u8 {
        self.ram_bank
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_rom(&mut self, address: usize, value: u8) {
        match address {
            0..=0x1FFF => {
                // toggle ram bank
                self.set_ram_enabled(value == 0x0A);
            }
            0x2000..=0x2FFF => {
                // change rom bank lower 8 bits
                self.rom_bank = u16::from(value);
            }
            0x3000..=0x3FFF => {
                // change rom bank higher bit
                self.rom_bank = self.rom_bank & 0xFF | (u16::from(value) << 8);
            }
            0x4000..=0x5FFF => {
                // change ram bank
                if self.ram_enabled {
                    self.ram_bank = value & 0xF;
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
        let mut mbc = MBC5::new(&[]);
        mbc.write_rom(0x2000, 4);
        assert_eq!(mbc.rom_bank(), 4);
    }

    #[test]
    fn rom_bank_switching_zero() {
        let mut mbc = MBC5::new(&[]);
        mbc.write_rom(0x2000, 0);
        assert_eq!(mbc.rom_bank(), 0);
    }

    #[test]
    fn enable_ram() {
        let mut mbc = MBC5::new(&[]);
        mbc.write_rom(0, 0x0A);
        assert!(mbc.ram_enabled());
    }

    #[test]
    fn ram_bank_default() {
        let mbc = MBC5::new(&[]);
        assert_eq!(mbc.relative_ram_address(0xA000), 0);
    }

    #[test]
    fn ram_bank_switching() {
        let mut mbc = MBC5::new(&[]);
        mbc.set_ram_enabled(true);
        mbc.write_rom(0x4000, 0);
        assert_eq!(mbc.ram_bank(), 0);

        mbc.write_rom(0x4000, 0x1F);
        assert_eq!(mbc.ram_bank(), 0x0F);
    }
}
