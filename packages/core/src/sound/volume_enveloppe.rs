#[derive(Default, Copy, Clone)]
pub struct VolumeEnveloppe {
    pub register: u8,
}

impl VolumeEnveloppe {
    pub fn initial_volume(self) -> u8 {
        self.register >> 4
    }

    pub fn direction(self) -> u8 {
        (self.register >> 3) & 1
    }

    pub fn enveloppe_sweep(self) -> u8 {
        self.register & 0b111
    }
}