use crate::sound::frequency_register::FrequencyRegister;
use crate::bus::{Readable, Writable};

#[derive(Default, Copy, Clone)]
pub struct WaveChannel {
    pub playback_register: u8,
    pub sound_length: u8,
    pub output_level: u8,
    pub frequency_register: FrequencyRegister,
    pub wave_pattern: [u8; 16]
}

impl WaveChannel {
    pub fn is_playing(self) -> bool {
        if self.playback_register >> 7 == 1 { true } else { false }
    }

    pub fn read_wave_pattern(&self, address: u16) -> u8 {
        let index = address as usize - 0xFF30;
        self.wave_pattern[index]
    }

    pub fn write_wave_pattern(&mut self, address: u16, value: u8) {
        let index = address as usize - 0xFF30;
        self.wave_pattern[index] = value;
    }
}

impl Readable for WaveChannel {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF1A => self.playback_register,
            0xFF1B => self.sound_length,
            0xFF1C => self.output_level,
            0xFF1D => { 0xFF } // Write-only address,
            0xFF1E => self.frequency_register.high_register,
            0xFF30..=0xFF3F => self.read_wave_pattern(address),
            _ => panic!("Illegal sound channel 3 read")
        }
    }
}

impl Writable for WaveChannel {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF1A => self.playback_register = value,
            0xFF1B => self.sound_length = value,
            0xFF1C => self.output_level = value,
            0xFF1D => self.frequency_register.low_register = value,
            0xFF1E => self.frequency_register.high_register = value,
            0xFF30..=0xFF3F => self.write_wave_pattern(address, value),
            _ => panic!("Illegal sound channel 3 read")
        }
    }
}