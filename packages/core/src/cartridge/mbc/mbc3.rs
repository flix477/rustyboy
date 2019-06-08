use super::real_time_clock::{RTCRegister, RealTimeClock};
use super::MemoryBankController;
use crate::cartridge::cartridge_capability::CartridgeCapability;
use std::cmp;

pub struct MBC3 {
    rom_bank: u8,
    ram_enabled: bool,
    ram_bank: u8,
    mode: MBC3Mode,
    clock: Option<RealTimeClock>,
}

impl MBC3 {
    pub fn new(capabilities: &[CartridgeCapability]) -> MBC3 {
        let clock = if capabilities.contains(&CartridgeCapability::Timer) {
            Some(RealTimeClock::new())
        } else {
            None
        };
        MBC3 {
            rom_bank: 1,
            ram_enabled: false,
            ram_bank: 0,
            mode: MBC3Mode::RAM,
            clock,
        }
    }

    pub fn mode(&self) -> &MBC3Mode {
        &self.mode
    }

    pub fn set_ram_enabled(&mut self, value: bool) {
        self.ram_enabled = value;
    }

    pub fn clock(&self) -> &Option<RealTimeClock> {
        &self.clock
    }
}

// TODO: implement RTC correctly
impl MemoryBankController for MBC3 {
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
                self.rom_bank = cmp::max(value & 0x7F, 1);
            }
            0x4000..=0x5FFF => {
                // change ram bank/rtc register
                match value {
                    0..=0x7 => {
                        // ram bank
                        self.mode = MBC3Mode::RAM;
                        self.ram_bank = value;
                    }
                    0x8..=0xC => {
                        // rtc register
                        if let (Some(clock), Some(value)) =
                            (&mut self.clock, RTCRegister::from_value(value))
                        {
                            self.mode = MBC3Mode::RTC;
                            clock.set_active_register(value);
                        }
                    }
                    _ => {}
                }
            }
            0x6000..=0x7FFF => {
                // latch clock data
                if let Some(clock) = &mut self.clock {
                    clock.latch();
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: usize, buffer: &[u8]) -> u8 {
        if let MBC3Mode::RAM = self.mode() {
            let address = self.relative_ram_address(address);
            buffer[address]
        } else if let Some(clock) = self.clock() {
            clock.active_value()
        } else {
            0 // TODO: i should really find a default for these
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MBC3Mode {
    RAM,
    RTC,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_bank_switching() {
        let mut mbc = MBC3::new(&[]);
        mbc.write_rom(0x2000, 0x7F);
        assert_eq!(mbc.rom_bank(), 0x7F);
    }

    #[test]
    fn rom_bank_switching_zero() {
        let mut mbc = MBC3::new(&[]);
        mbc.write_rom(0x2000, 0);
        assert_eq!(mbc.rom_bank(), 1);
    }

    #[test]
    fn set_mode() {
        let mut mbc = MBC3::new(&[CartridgeCapability::Timer]);

        mbc.write_rom(0x4000, 8);
        assert_eq!(mbc.mode(), &MBC3Mode::RTC);

        mbc.write_rom(0x4000, 7);
        assert_eq!(mbc.mode(), &MBC3Mode::RAM);
    }

    #[test]
    fn enable_ram() {
        let mut mbc = MBC3::new(&[]);
        mbc.write_rom(0, 0x0A);
        assert!(mbc.ram_enabled());
    }

    #[test]
    fn ram_bank_default() {
        let mbc = MBC3::new(&[]);
        assert_eq!(mbc.relative_ram_address(0xA000), 0);
    }

    #[test]
    fn ram_bank_switching() {
        let mut mbc = MBC3::new(&[]);
        mbc.set_ram_enabled(true);
        mbc.write_rom(0x4000, 7);
        assert_eq!(mbc.ram_bank(), 7);
    }

    #[test]
    fn rtc_register_switching() {
        let mut mbc = MBC3::new(&[CartridgeCapability::Timer]);
        mbc.set_ram_enabled(true);
        mbc.write_rom(0x4000, 8);
        if let Some(clock) = mbc.clock() {
            assert_eq!(clock.active_register(), &RTCRegister::Seconds);
        }
    }
}
