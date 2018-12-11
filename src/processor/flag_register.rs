use crate::processor::register::{Register, SingleRegister, DualRegister};
use crate::util::bitflags::Bitflags;

pub struct FlagRegister {
    pub register: DualRegister
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        return FlagRegister {
            register: DualRegister::new()
        };
    }

    pub fn accumulator(&self) -> &SingleRegister {
        &self.register.high
    }

    pub fn flags(&self) -> &SingleRegister {
        &self.register.low
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.register.high.set(value as u16);
    }

    pub fn set_flags(&mut self, value: u8) {
        self.register.low.set(value as u16);
    }

    pub fn register(&self) -> &DualRegister {
        &self.register
    }
}

impl Bitflags<Flag> for FlagRegister {
    fn register(&self) -> u8 {
        self.register.low.get() as u8
    }

    fn set_register(&mut self, value: u8) {
        self.register.low.set(value as u16);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Flag {
    Carry = 16, // c, set when an addition becomes bigger than 0xFF or 0xFFFF
    HalfCarry = 32, // h
    AddSub = 64, // n
    Zero = 128 // z, set when an operation has been zero
}

impl Into<u8> for Flag {
    fn into(self) -> u8 {
        self as u8
    }
}