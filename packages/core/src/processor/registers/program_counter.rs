use super::register::Register;
use crate::bus::Readable;

#[derive(Copy, Clone)]
pub struct ProgramCounter {
    pub value: u16,
}

impl ProgramCounter {
    pub fn new(value: u16) -> ProgramCounter {
        Self { value }
    }

    pub fn fetch<H: Readable>(&mut self, bus: &H) -> u8 {
        let value = bus.read(self.value);
        self.increment();
        value
    }

    pub fn peek<H: Readable>(self, bus: &H) -> u8 {
        bus.read(self.value)
    }

    pub fn peek16<H: Readable>(self, bus: &H) -> u16 {
        (u16::from(bus.read(self.value + 1)) << 8) | u16::from(bus.read(self.value))
    }
}

impl Default for ProgramCounter {
    fn default() -> ProgramCounter {
        ProgramCounter { value: 0x100 }
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
