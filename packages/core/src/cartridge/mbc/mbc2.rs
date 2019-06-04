use super::MemoryBankController;
use crate::cartridge::cartridge_capability::CartridgeCapability;
use std::cmp;

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

impl MemoryBankController for MBC2 {
    fn rom_bank(&self) -> u16 {
        self.rom_bank as u16
    }

    fn ram_bank(&self) -> u8 {
        0
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_rom(&mut self, address: usize, value: u8) {
        match address {
            0...0x1FFF => {
                // toggle ram bank
                if address & 0x100 == 0 {
                    self.ram_enabled = value == 0x0A;
                }
            }
            0x2000...0x3FFF => {
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
        let mut mbc = MBC2::new(&vec![]);

        // shouldn't work
        mbc.write_rom(0x2000, 15);
        assert_ne!(mbc.rom_bank(), 15);

        // should work
        mbc.write_rom(0x2100, 15);
        assert_eq!(mbc.rom_bank(), 15);
    }

    #[test]
    fn rom_bank_switching_zero() {
        let mut mbc = MBC2::new(&vec![]);
        mbc.write_rom(0x2100, 0);
        assert_eq!(mbc.rom_bank(), 1);
    }

    #[test]
    fn enable_ram() {
        let mut mbc = MBC2::new(&vec![]);

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
