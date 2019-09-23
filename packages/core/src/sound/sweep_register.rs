#[derive(Default, Copy, Clone)]
pub struct SweepRegister {
    pub register: u8
}

// 000 0000

impl SweepRegister {
    pub fn sweep_time(self) -> u8 {
        (self.register >> 4) & 0b111
    }

    pub fn sweep_modifier(self) -> u8 {
        (self.register >> 3) & 1
    }

    pub fn is_increasing(self) -> bool {
        if self.sweep_modifier() == 0 { true } else { false }
    }

    pub fn sweep_shift(self) -> u8 {
        self.register & 0b111
    }

    /// 000: sweep off - no freq change
    /// 001: 7.8 ms  (1/128Hz)
    /// 010: 15.6 ms (2/128Hz)
    /// 011: 23.4 ms (3/128Hz)
    /// 100: 31.3 ms (4/128Hz)
    /// 101: 39.1 ms (5/128Hz)
    /// 110: 46.9 ms (6/128Hz)
    /// 111: 54.7 ms (7/128Hz)
    pub fn computed_sweep_time(self) -> f32 {
        unimplemented!()
    }
}

pub enum SweepModifier {
    Increase,
    Decrease
}