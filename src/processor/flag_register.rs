use processor::register::DualRegister;

pub struct FlagRegister {
    register: DualRegister
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        return FlagRegister {
            register: DualRegister::new()
        };
    }

    pub fn accumulator(&self) -> u8 {
        return self.register.high();
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.register.set_high(value);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        return self.register.low() & (flag as u8) != 0;
    }

    pub fn set_flag(&mut self, flag: Flag) {
        let new_low = self.register.low() & !(flag as u8);
        self.register.set_low(new_low);
    }
}

pub enum Flag {
    Carry = 16, // c, set when an addition becomes bigger than 0xFF or 0xFFFF
    HalfCarry = 32, // h
    AddSub = 64, // n
    Zero = 128 // z, set when an operation has been zero
}