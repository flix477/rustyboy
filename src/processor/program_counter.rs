use crate::bus::Bus;
use crate::processor::register::Register;

pub struct ProgramCounter {
    value: u16,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { value: 0x100 }
    }

    pub fn fetch<H: Bus>(&mut self, bus: &H) -> u8 {
        let value = bus.read(self.value);
        self.increment();
        value
    }

    pub fn peek<H: Bus>(&mut self, bus: &H) -> u8 {
        bus.read(self.value)
    }

    pub fn peek16<H: Bus>(&mut self, bus: &H) -> u16 {
        ((bus.read(self.value + 1) as u16) << 8) | (bus.read(self.value) as u16)
    }
}

impl Register for ProgramCounter {
    fn get(&self) -> u16 {
        self.value
    }

    fn set(&mut self, value: u16) {
        self.value = value
    }

    fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }
}
