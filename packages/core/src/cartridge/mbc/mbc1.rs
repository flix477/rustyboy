use super::MemoryBankController;
use crate::cartridge::cartridge_capability::CartridgeCapability;
use crate::util::savestate::{Savestate, read_savestate_bool, read_savestate_byte, LoadSavestateError};

pub struct MBC1 {
    mode: MBC1Mode,
    ram_enabled: bool,
    register: u8,
}

impl MBC1 {
    pub fn new(_capabilities: &[CartridgeCapability]) -> MBC1 {
        MBC1 {
            mode: MBC1Mode::MaxROM,
            ram_enabled: false,
            register: 1,
        }
    }

    pub fn mode(&self) -> &MBC1Mode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: MBC1Mode) {
        self.mode = mode;
    }

    pub fn set_ram_enabled(&mut self, enabled: bool) {
        self.ram_enabled = enabled;
    }

    fn set_rom_bank(&mut self, value: u8) {
        let mask = self.rom_bank_mask();
        let value = if value == 0x60 || value == 0x40 || value == 0x20 || value == 0 {
            value + 1
        } else {
            value
        };
        self.register = (!mask & self.register) | (value & mask);
    }

    fn set_ram_bank(&mut self, value: u8) {
        let mask = self.rom_bank_mask();
        let value = if self.mode == MBC1Mode::MaxROM {
            0
        } else {
            value & 3
        } << 5;

        self.register = (self.register & mask) | value;
    }

    fn rom_bank_mask(&self) -> u8 {
        if self.mode == MBC1Mode::MaxRAM {
            0b1_1111
        } else {
            0xFF
        }
    }
}

impl Savestate for MBC1 {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.mode as u8);
        buffer.push(self.ram_enabled as u8);
        buffer.push(self.register);
    }

    fn load_savestate<'a>(&mut self, buffer: &mut std::slice::Iter<u8>) -> Result<(), LoadSavestateError> {
        self.mode = buffer.next().cloned().and_then(MBC1Mode::from).ok_or(LoadSavestateError::InvalidSavestate)?;
        self.ram_enabled = read_savestate_bool(buffer)?;
        self.register = read_savestate_byte(buffer)?;
        Ok(())
    }
}

impl MemoryBankController for MBC1 {
    fn rom_bank(&self) -> u16 {
        let mask = self.rom_bank_mask();
        u16::from(self.register & mask)
    }

    fn ram_bank(&self) -> u8 {
        if self.mode == MBC1Mode::MaxROM {
            0
        } else {
            self.register >> 5
        }
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
            0x2000..=0x3FFF => {
                // change rom bank
                self.set_rom_bank((value & 0b1_1111) | (self.rom_bank() as u8 & 0b110_0000));
            }
            0x4000..=0x5FFF => {
                let value = value & 0b11;
                // change ram bank/rom bank set
                if let MBC1Mode::MaxRAM = self.mode() {
                    if self.ram_enabled {
                        self.set_ram_bank(value);
                    }
                } else {
                    self.set_rom_bank((self.rom_bank() as u8 & 0b1_1111) | (value << 5));
                }
            }
            0x6000..=0x7FFF => {
                // change mode
                self.set_mode(if value & 1 == 0 {
                    MBC1Mode::MaxROM
                } else {
                    MBC1Mode::MaxRAM
                });
            }
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MBC1Mode {
    MaxROM,
    MaxRAM,
}

impl MBC1Mode {
    pub fn from(value: u8) -> Option<MBC1Mode> {
        match value {
            0 => Some(MBC1Mode::MaxROM),
            1 => Some(MBC1Mode::MaxRAM),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_bank_switching() {
        let mut mbc = MBC1::new(&[]);
        mbc.write_rom(0x2000, 4);
        assert_eq!(mbc.rom_bank(), 4);
    }

    #[test]
    fn rom_bank_switching_zero() {
        let mut mbc = MBC1::new(&[]);
        mbc.write_rom(0x2000, 0);
        assert_eq!(mbc.rom_bank(), 1);
    }

    #[test]
    fn rom_bank_set_switching() {
        let mut mbc = MBC1::new(&[]);
        mbc.write_rom(0x4000, 3);
        assert_eq!(mbc.rom_bank(), 0b0110_0001);
    }

    #[test]
    fn set_mode() {
        let mut mbc = MBC1::new(&[]);

        mbc.write_rom(0x6000, 0);
        assert_eq!(mbc.mode(), &MBC1Mode::MaxROM);

        mbc.write_rom(0x6000, 1);
        assert_eq!(mbc.mode(), &MBC1Mode::MaxRAM);
    }

    #[test]
    fn enable_ram() {
        let mut mbc = MBC1::new(&[]);
        mbc.write_rom(0, 0x0A);
        assert!(mbc.ram_enabled());
    }

    #[test]
    fn ram_bank_default() {
        let mbc = MBC1::new(&[]);
        assert_eq!(mbc.relative_ram_address(0xA000), 0);
    }

    #[test]
    fn ram_bank_switching() {
        let mut mbc = MBC1::new(&[]);
        mbc.set_mode(MBC1Mode::MaxRAM);
        mbc.set_ram_enabled(true);
        mbc.write_rom(0x4000, 0);
        assert_eq!(mbc.ram_bank(), 0);

        mbc.write_rom(0x4000, 3);
        assert_eq!(mbc.ram_bank(), 3);
    }
}
