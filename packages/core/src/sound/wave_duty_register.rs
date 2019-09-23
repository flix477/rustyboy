#[derive(Default, Copy, Clone)]
pub struct WaveDutyRegister {
    pub register: u8
}

impl WaveDutyRegister {
    /// 00: 12.5% ( _-------_-------_------- )
    /// 01: 25%   ( __------__------__------ )
    /// 10: 50%   ( ____----____----____---- ) (normal)
    /// 11: 75%   ( ______--______--______-- )
    pub fn duty(self) -> u8 {
        self.register >> 6
    }

    pub fn sound_length(self) -> u8 {
        self.register & 0b11_1111
    }
}