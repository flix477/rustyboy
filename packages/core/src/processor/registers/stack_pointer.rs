use super::register::Register;
use crate::bus::Bus;

#[derive(Copy, Clone)]
pub struct StackPointer {
    value: u16,
}

impl StackPointer {
    pub fn new() -> StackPointer {
        Self::default()
    }

    pub fn peek<H: Bus>(self, bus: &H) -> u8 {
        bus.read(self.value)
    }

    pub fn push<H: Bus>(&mut self, bus: &mut H, value: u16) {
        self.decrement();
        bus.write(self.value, (value >> 8) as u8);
        self.decrement();
        bus.write(self.value, value as u8);
    }

    pub fn pop<H: Bus>(&mut self, bus: &H) -> u16 {
        let low = u16::from(bus.read(self.value));
        self.increment();
        let high = u16::from(bus.read(self.value));
        self.increment();
        low | (high << 8)
    }
}

impl Default for StackPointer {
    fn default() -> StackPointer {
        StackPointer { value: 0xFFFE }
    }
}

impl Register for StackPointer {
    fn get(&self) -> u16 {
        self.value
    }

    fn set(&mut self, value: u16) {
        self.value = value;
    }

    fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }
}
