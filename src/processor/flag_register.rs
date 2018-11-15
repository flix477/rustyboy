use processor::register::{Register, SingleRegister, DualRegister};

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

    pub fn flag(&self, flag: Flag) -> bool {
        return self.register.low.get() as u8 & (flag as u8) != 0;
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let mut new_low = self.register.low.get() as u8;
        if value {
            new_low |= flag as u8;
        } else {
            new_low &= !(flag as u8);
        }
        self.register.low.set(new_low as u16);
    }

    pub fn set_flags(&mut self, value: u8) {
        self.register.low.set(value as u16);
    }

    pub fn register(&self) -> &DualRegister {
        &self.register
    }

    pub fn set_zero_from_result(&mut self, result: u8) {
        self.set_flag(Flag::Zero, result == 0);
    }
}

pub enum Flag {
    Carry = 16, // c, set when an addition becomes bigger than 0xFF or 0xFFFF
    HalfCarry = 32, // h
    AddSub = 64, // n
    Zero = 128 // z, set when an operation has been zero
}