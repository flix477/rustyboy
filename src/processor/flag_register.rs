use processor::register::{SingleRegister, DualRegister};

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
        self.register.high.set(value);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        return self.register.low.get() & (flag as u8) != 0;
    }

    pub fn set_flag(&mut self, flag: Flag) {
        let new_low = self.register.low.get() & !(flag as u8);
        self.register.low.set(new_low);
    }

    pub fn set_flags(&mut self, value: u8) {
        self.register.low.set(value);
    }

    pub fn register(&self) -> &DualRegister {
        &self.register
    }
}

pub enum Flag {
    Carry = 16, // c, set when an addition becomes bigger than 0xFF or 0xFFFF
    HalfCarry = 32, // h
    AddSub = 64, // n
    Zero = 128 // z, set when an operation has been zero
}