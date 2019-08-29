use super::color::Color;
use crate::util::savestate::{read_savestate_byte, LoadSavestateError, Savestate};

#[derive(Copy, Clone)]
pub struct Palette {
    register: u8,
}

impl Palette {
    pub fn from_value(value: u8) -> Palette {
        Palette { register: value }
    }

    pub fn color(self, idx: u8) -> Color {
        Color::from((self.register >> (2 * idx)) & 0b11)
    }

    pub fn get(self) -> u8 {
        self.register
    }

    pub fn set(&mut self, value: u8) {
        self.register = value;
    }
}

impl Savestate for Palette {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.register);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<'a, u8>,
    ) -> Result<(), LoadSavestateError> {
        self.register = read_savestate_byte(buffer)?;
        Ok(())
    }
}
