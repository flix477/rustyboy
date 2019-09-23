#[derive(Copy, Clone, Default)]
pub struct OutputRegister {
    pub register: u8
}

impl OutputRegister {
    pub fn channel1_pan(self) -> (u8, u8) {
        (
            self.register & 1,
            self.register >> 4 & 1
        )
    }

    pub fn channel2_pan(self) -> (u8, u8) {
        (
            self.register >> 1 & 1,
            self.register >> 5 & 1
        )
    }

    pub fn channel3_pan(self) -> (u8, u8) {
        (
            self.register >> 2 & 1,
            self.register >> 6 & 1
        )
    }

    pub fn channel4_pan(self) -> (u8, u8) {
        (
            self.register >> 3 & 1,
            self.register >> 7 & 1
        )
    }

    pub fn panning(self) -> [(u8, u8); 4] {
        [
            self.channel1_pan(),
            self.channel2_pan(),
            self.channel3_pan(),
            self.channel4_pan(),
        ]
    }
}