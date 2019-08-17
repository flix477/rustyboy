mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod real_time_clock;

use crate::cartridge::cartridge_capability::CartridgeCapability;
use crate::cartridge::cartridge_metadata::CartridgeMetadata;
use crate::cartridge::mbc::mbc1::MBC1;
use crate::cartridge::mbc::mbc2::MBC2;
use crate::cartridge::mbc::mbc3::MBC3;
use crate::cartridge::mbc::mbc5::MBC5;

pub struct MBCFactory;
impl MBCFactory {
    pub fn from_metadata(metadata: &CartridgeMetadata) -> Option<Box<dyn MemoryBankController>> {
        let variant = MBCVariant::from_capabilities(&metadata.capabilities)?;
        Self::from_variant(&variant, &metadata.capabilities)
    }

    pub fn from_variant(
        variant: &MBCVariant,
        capabilities: &[CartridgeCapability],
    ) -> Option<Box<dyn MemoryBankController>> {
        match variant {
            MBCVariant::MBC1 => Some(Box::new(MBC1::new(capabilities))),
            MBCVariant::MBC2 => Some(Box::new(MBC2::new(capabilities))),
            MBCVariant::MBC3 => Some(Box::new(MBC3::new(capabilities))),
            MBCVariant::MBC5 => Some(Box::new(MBC5::new(capabilities))),
        }
    }
}

pub trait MemoryBankController {
    fn rom_bank(&self) -> u16;
    fn ram_bank(&self) -> u8;
    fn ram_enabled(&self) -> bool;

    fn relative_rom_address(&self, address: usize) -> usize {
        let current_bank = self.rom_bank().saturating_sub(1) as usize;
        address + current_bank * 0x4000
    }

    fn write_rom(&mut self, _address: usize, _value: u8) {}

    fn read_ram(&self, address: usize, ram: &[u8]) -> u8 {
        let address = self.relative_ram_address(address);
        if address < ram.len() {
            ram[address]
        } else {
            0xFF
        }
    }

    fn write_ram(&self, address: usize, value: u8, ram: &mut [u8]) {
        let address = self.relative_ram_address(address);
        if address < ram.len() {
            ram[address] = value
        }
    }

    fn relative_ram_address(&self, address: usize) -> usize {
        let current_bank = self.ram_bank() as usize;
        address + current_bank * 0x2000 - 0xA000
    }
}

pub enum MBCVariant {
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

impl MBCVariant {
    pub fn from_capabilities(capabilities: &[CartridgeCapability]) -> Option<MBCVariant> {
        if capabilities.contains(&CartridgeCapability::MBC1) {
            Some(MBCVariant::MBC1)
        } else if capabilities.contains(&CartridgeCapability::MBC2) {
            Some(MBCVariant::MBC2)
        } else if capabilities.contains(&CartridgeCapability::MBC3) {
            Some(MBCVariant::MBC3)
        } else if capabilities.contains(&CartridgeCapability::MBC5) {
            Some(MBCVariant::MBC5)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static MBC_VARIANTS: [MBCVariant; 4] = [
        MBCVariant::MBC1,
        MBCVariant::MBC2,
        MBCVariant::MBC3,
        MBCVariant::MBC5,
    ];

    #[test]
    fn rom_bank_defaults() {
        for variant in MBC_VARIANTS.iter() {
            if let Some(mbc) = MBCFactory::from_variant(variant, &[]) {
                assert_eq!(mbc.relative_rom_address(0x4000), 0x4000);
            }
        }
    }

}
