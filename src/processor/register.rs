pub struct Register {
    pub value: u8
}

impl Register {
    pub fn new() -> Register {
        return Register {
            value: 0
        };
    }

    pub fn get(&self, idx: u8) -> bool {
        return self.value >> (7 - idx) == 1;
    }

    pub fn set(&mut self, idx: u8, value: bool) {
        let padded_value = (value as u8) << idx;
        if value {
            self.value |= padded_value;
        } else {
            self.value &= !2u8.pow(idx as u32);
        }
    }
}

pub struct DualRegister {
     value: u16
}

impl DualRegister {
    pub fn new() -> DualRegister {
        return DualRegister {
            value: 0
        };
    }

    pub fn value(&self) -> u16 {
        return self.value;
    }

    pub fn high(&self) -> u8 {
        return (self.value >> 8) as u8;
    }

    pub fn low(&self) -> u8 {
        return self.value as u8;
    }

    pub fn set_value(&mut self, value: u16) {
        self.value = value;
    }

    pub fn set_high(&mut self, value: u8) {
        self.value = self.low() as u16 | ((value as u16) << 8);
    }

    pub fn set_low(&mut self, value: u8) {
        self.value = ((self.high() as u16) << 8) | value as u16;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_high() { // lol
        let mut register = DualRegister::new();
        register.set_value(0b1010101001010101);
        assert_eq!(register.high(), 0b10101010);
    }

    #[test]
    fn get_low() {
        let mut register = DualRegister::new();
        register.set_value(0b1010101001010101);
        assert_eq!(register.low(), 0b01010101);
    }

    #[test]
    fn set_high() {
        let mut register = DualRegister::new();
        register.set_high(0b01010101);
        assert_eq!(register.high(), 0b01010101);
    }

    #[test]
    fn set_low() {
        let mut register = DualRegister::new();
        register.set_low(0b01010101);
        assert_eq!(register.low(), 0b01010101);
    }

    #[test]
    fn set() {
        let mut register = Register::new();
        register.value = 0b10101010;
        register.set(1, false);
        assert_eq!(register.value, 0b10101000);
    }
}