use super::MemoryBankController;
use std::cmp;
use cartridge::cartridge_capability::CartridgeCapability;

pub struct MBC5 {
    rom_bank: u16,
    ram_enabled: bool,
    ram_bank: u8
}

impl MBC5 {
    pub fn new(capabilities: &[CartridgeCapability]) -> MBC5 {
//        let has_ram = capabilities.contains(&CartridgeCapability::RAM);
        MBC5 {
            rom_bank: 0,
            ram_enabled: false,
            ram_bank: 0
        }
    }

    pub fn set_ram_enabled(&mut self, enabled: bool) {
        self.ram_enabled = enabled;
    }
}

impl MemoryBankController for MBC5 {
    fn rom_bank(&self) -> u16 { self.rom_bank }

    fn ram_bank(&self) -> u8 { self.ram_bank }

    fn ram_enabled(&self) -> bool { self.ram_enabled }

    fn write_rom(&mut self, address: usize, value: u8) {
        match address {
            0...0x1FFF => { // toggle ram bank
                self.ram_enabled = value == 0x0A;
            },
            0x2000...0x2FFF => { // change rom bank lower 8 bits
                self.rom_bank = value as u16;
            },
            0x3000...0x3FFF => { // change rom bank higher bit
                self.rom_bank = self.rom_bank & 0xFF | ((value as u16) << 8);
            },
            0x4000...0x5FFF => { // change ram bank
                if self.ram_enabled {
                    self.ram_bank = value & 0xF;
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MBC5Mode {
    MaxROM,
    MaxRAM
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_bank_switching() {
        let mut mbc = MBC5::new(&vec![]);
        mbc.write_rom(0x2000, 4);
        assert_eq!(mbc.rom_bank(), 4);
    }

    #[test]
    fn rom_bank_switching_zero() {
        let mut mbc = MBC5::new(&vec![]);
        mbc.write_rom(0x2000, 0);
        assert_eq!(mbc.rom_bank(), 0);
    }

    #[test]
    fn enable_ram() {
        let mut mbc = MBC5::new(&vec![]);
        mbc.write_rom(0, 0x0A);
        assert!(mbc.ram_enabled());
    }

    #[test]
    fn ram_bank_default() {
        let mbc = MBC5::new(&vec![]);
        assert_eq!(mbc.relative_ram_address(0xA000), 0);
    }

    #[test]
    fn ram_bank_switching() {
        let mut mbc = MBC5::new(&vec![]);
        mbc.set_ram_enabled(true);
        mbc.write_rom(0x4000, 0);
        assert_eq!(mbc.ram_bank(), 0);

        mbc.write_rom(0x4000, 0x1F);
        assert_eq!(mbc.ram_bank(), 0x0F);
    }
}
