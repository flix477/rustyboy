use super::register::{DualRegister, Register, SingleRegister};
use crate::util::bitflags::Bitflags;

#[derive(Copy, Clone)]
pub struct FlagRegister {
    register: DualRegister,
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        Self::default()
    }

    pub fn accumulator(&self) -> &SingleRegister {
        &self.register.high
    }

    pub fn flags(&self) -> &SingleRegister {
        &self.register.low
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.register.high.set(u16::from(value));
    }

    pub fn set_flags(&mut self, value: u8) {
        self.register.low.set(u16::from(value) & 0xF0);
    }

    pub fn register(&self) -> &DualRegister {
        &self.register
    }

    pub fn set(&mut self, value: u16) {
        self.set_flags(value as u8);
        self.set_accumulator((value >> 8) as u8);
    }
}

impl Default for FlagRegister {
    fn default() -> FlagRegister {
        FlagRegister {
            register: DualRegister::from(0x01B0),
        }
    }
}

impl Bitflags<Flag> for FlagRegister {
    fn register(&self) -> u8 {
        self.register.low.get() as u8
    }

    fn set_register(&mut self, value: u8) {
        self.register.low.set(u16::from(value));
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Flag {
    Carry = 16,     // c, set when an addition becomes bigger than 0xFF or 0xFFFF
    HalfCarry = 32, // h
    AddSub = 64,    // n
    Zero = 128,     // z, set when an operation has been zero
}

impl ToString for Flag {
    fn to_string(&self) -> String {
        match *self {
            Flag::Carry => "c".to_string(),
            Flag::HalfCarry => "h".to_string(),
            Flag::AddSub => "n".to_string(),
            Flag::Zero => "z".to_string(),
        }
    }
}

impl Into<u8> for Flag {
    fn into(self) -> u8 {
        self as u8
    }
}

pub const fn half_carry_add(value1: u8, value2: u8) -> bool {
    (((value1 & 0xf) + (value2 & 0xf)) & 0x10) == 0x10
}

pub const fn half_carry_add16(value1: u16, value2: u16) -> bool {
    (((value1 & 0xfff) + (value2 & 0xfff)) & 0x1000) == 0x1000
}

pub const fn half_carry_sub(value1: u8, value2: u8) -> bool {
    value1 & 0xF < value2 & 0xF
}

pub fn carry_add(value1: u8, value2: u8) -> bool {
    value1.overflowing_add(value2).1
}
