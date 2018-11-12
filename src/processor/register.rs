pub struct SingleRegister {
    value: u8
}

impl SingleRegister {
    pub fn new() -> SingleRegister {
        return SingleRegister {
            value: 0
        };
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get_bit(&self, idx: u8) -> bool {
        return self.value >> (7 - idx) == 1;
    }

    pub fn set_bit(&mut self, idx: u8, value: bool) {
        let padded_value = (value as u8) << idx;
        if value {
            self.value |= padded_value;
        } else {
            self.value &= !2u8.pow(idx as u32);
        }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn decrement(&mut self) {
        self.value += 1;
    }
}

pub struct DualRegister {
    pub high: SingleRegister,
    pub low: SingleRegister
}

impl DualRegister {
    pub fn new() -> DualRegister {
        return DualRegister {
            high: SingleRegister::new(),
            low: SingleRegister::new()
        };
    }

    pub fn get(&self) -> u16 {
        return (self.high.get() as u16) | self.low.get() as u16;
    }

    pub fn set(&mut self, value: u16) {
        self.low.set(value as u8);
        self.high.set((value >> 8) as u8);
    }

    pub fn increment(&mut self) {
        let current_value = self.get();
        self.set(current_value + 1);
    }

    pub fn decrement(&mut self) {
        let current_value = self.get();
        self.set(current_value - 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_high() { // lol
        let mut register = DualRegister::new();
        register.set(0b1010101001010101);
        assert_eq!(register.high.get(), 0b10101010);
    }

    #[test]
    fn get_low() {
        let mut register = DualRegister::new();
        register.set(0b1010101001010101);
        assert_eq!(register.low.get(), 0b01010101);
    }

    #[test]
    fn set_high() {
        let mut register = DualRegister::new();
        register.high.set(0b01010101);
        assert_eq!(register.high.get(), 0b01010101);
    }

    #[test]
    fn set_low() {
        let mut register = DualRegister::new();
        register.low.set(0b01010101);
        assert_eq!(register.low.get(), 0b01010101);
    }

    #[test]
    fn set() {
        let mut register = SingleRegister::new();
        register.set(0b10101010);
        register.set_bit(1, false);
        assert_eq!(register.get(), 0b10101000);
    }
}