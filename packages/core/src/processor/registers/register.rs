pub trait Register {
    fn get(&self) -> u16;
    fn set(&mut self, value: u16);
    fn increment(&mut self);
    fn decrement(&mut self);
}

#[derive(Default, Copy, Clone)]
pub struct SingleRegister {
    value: u8,
}

impl SingleRegister {
    pub fn new() -> SingleRegister {
        SingleRegister::default()
    }

    pub fn get_bit(self, idx: u8) -> bool {
        self.value >> (7 - idx) == 1
    }

    pub fn set_bit(&mut self, idx: u8, value: bool) {
        let padded_value = (value as u8) << idx;
        if value {
            self.value |= padded_value;
        } else {
            self.value &= !2u8.pow(u32::from(idx));
        }
    }
}

impl Register for SingleRegister {
    fn get(&self) -> u16 {
        u16::from(self.value)
    }

    fn set(&mut self, value: u16) {
        self.value = value as u8;
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn decrement(&mut self) {
        self.value -= 1;
    }
}

#[derive(Default, Copy, Clone)]
pub struct DualRegister {
    pub high: SingleRegister,
    pub low: SingleRegister,
}

impl DualRegister {
    pub fn new() -> DualRegister {
        DualRegister::default()
    }

    pub fn from(value: u16) -> DualRegister {
        let mut reg = DualRegister {
            high: SingleRegister::new(),
            low: SingleRegister::new(),
        };
        reg.set(value);
        reg
    }
}

impl Register for DualRegister {
    fn get(&self) -> u16 {
        ((self.high.get() as u16) << 8) | self.low.get() as u16
    }

    fn set(&mut self, value: u16) {
        self.low.set(value);
        self.high.set(value >> 8);
    }

    fn increment(&mut self) {
        let current_value = self.get();
        self.set(current_value + 1);
    }

    fn decrement(&mut self) {
        let current_value = self.get();
        self.set(current_value - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_high() {
        // lol
        let mut register = DualRegister::new();
        register.set(0b1010_1010_0101_0101);
        assert_eq!(register.high.get(), 0b1010_1010);
    }

    #[test]
    fn get_low() {
        let mut register = DualRegister::new();
        register.set(0b1010_1010_0101_0101);
        assert_eq!(register.low.get(), 0b0101_0101);
    }

    #[test]
    fn set_high() {
        let mut register = DualRegister::new();
        register.high.set(0b0101_0101);
        assert_eq!(register.high.get(), 0b0101_0101);
    }

    #[test]
    fn set_low() {
        let mut register = DualRegister::new();
        register.low.set(0b0101_0101);
        assert_eq!(register.low.get(), 0b0101_0101);
    }

    #[test]
    fn set() {
        let mut register = SingleRegister::new();
        register.set(0b1010_1010);
        register.set_bit(1, false);
        assert_eq!(register.get(), 0b1010_1000);
    }
}
