use crate::util::bits::get_bit;
use crate::video::Video;

#[derive(Default)]
pub struct StatusRegister {
    pub register: u8,
}

// TODO: bunch of stuff
impl StatusRegister {
    pub fn generate(&self, video: &Video) -> u8 {
        let mode = video.mode as u8;
        let coincidence_flag =
            (video.position_registers.ly() == video.position_registers.lyc()) as u8;
        (self.register & 0b1111_1000) | (coincidence_flag << 2) | mode
    }

    pub fn set(&mut self, value: u8) {
        self.register = (self.register & 0b111) | (value & 0b1111_1000);
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

impl StatusMode {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(StatusMode::HBlank),
            1 => Some(StatusMode::VBlank),
            2 => Some(StatusMode::ReadingOAM),
            3 => Some(StatusMode::LCDTransfer),
            _ => None,
        }
    }
}

pub enum InterruptCondition {
    LYCEquality = 0b100_0000,
    OAM = 0b10_0000,
    VBlank = 0b1_0000,
    HBlank = 0b1000,
}
