#[derive(Copy, Clone, Default)]
pub struct FrequencyRegister {
    pub low_register: u8,
    pub high_register: u8,
}

impl FrequencyRegister {
    pub fn frequency(self) -> u16 {
        u16::from(self.high_register & 0b11) << 8 | u16::from(self.low_register)
    }

    /// Whether the output stops once the sound length is attained
    pub fn is_finite(self) -> bool {
        if self.high_register >> 6 & 1 == 1 { true } else { false }
    }

    pub fn computed_frequency(self) -> f32 {
        131072.0 / (2048.0 - self.frequency() as f32)
    }
}