use crate::util::bits::get_bit;
use crate::video::Video;

pub struct StatusRegister {
    register: u8,
}

// TODO: bunch of stuff

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister { register: 0 }
    }

    pub fn generate(&self, video: &Video) -> u8 {
        let mode = video.mode as u8;
        let coincidence_flag = (video.ly == video.lyc) as u8;
        (self.register & 0b11111000) | (coincidence_flag << 2) | mode
    }

    pub fn set(&mut self, value: u8) {
        self.register = (self.register & 0b111) | (value & 0b11111000);
    }

    pub fn lyc_interrupt_enabled(&self) -> bool {
        get_bit(self.register, 6)
    }

    pub fn oam_interrupt_enabled(&self) -> bool {
        get_bit(self.register, 5)
    }

    pub fn vblank_interrupt_enabled(&self) -> bool {
        get_bit(self.register, 4)
    }

    pub fn hblank_interrupt_enabled(&self) -> bool {
        get_bit(self.register, 3)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatusMode {
    HBlank = 0,
    VBlank = 1,
    ReadingOAM = 2,
    LCDTransfer = 3,
}

impl From<u8> for StatusMode {
    fn from(value: u8) -> Self {
        match value {
            0 => StatusMode::HBlank,
            1 => StatusMode::VBlank,
            2 => StatusMode::ReadingOAM,
            3 => StatusMode::LCDTransfer,
            _ => panic!("Invalid value."),
        }
    }
}

pub enum InterruptCondition {
    LYCEquality = 0b1000000,
    OAM = 0b100000,
    VBlank = 0b10000,
    HBlank = 0b1000,
}
