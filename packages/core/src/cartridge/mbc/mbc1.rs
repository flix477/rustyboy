use super::MemoryBankController;
use crate::cartridge::cartridge_capability::CartridgeCapability;
use std::cmp;

pub struct MBC1 {
    mode: MBC1Mode,
    rom_bank: u8,
    ram_enabled: bool,
    ram_bank: u8,
}

impl MBC1 {
    pub fn new(_capabilities: &[CartridgeCapability]) -> MBC1 {
        // let has_ram = capabilities.contains(&CartridgeCapability::RAM);
        MBC1 {
            mode: MBC1Mode::MaxROM,
            rom_bank: 1,
            ram_enabled: false,
            ram_bank: 0,
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
}

impl MemoryBankController for MBC1 {
    fn rom_bank(&self) -> u16 {
        u16::from(self.rom_bank)
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
            0x2000..=0x3FFF => {
                // change rom bank
                self.rom_bank = cmp::max(value & 0b1_1111, 1) | (self.rom_bank & 0b110_0000);
            }
            0x4000..=0x5FFF => {
                // change ram bank/rom bank set
                if let MBC1Mode::MaxRAM = self.mode() {
                    if self.ram_enabled {
                        self.ram_bank = value & 0b11;
                    }
                } else {
                    self.rom_bank = (self.rom_bank & 0b1_1111) | (value << 5);
                }
            }
            0x6000..=0x7FFF => {
                // change mode
                self.set_mode(if value == 1 {
                    MBC1Mode::MaxRAM
                } else {
                    MBC1Mode::MaxROM
                });
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MBC1Mode {
    MaxROM,
    MaxRAM,
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
