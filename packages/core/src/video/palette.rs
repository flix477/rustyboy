use super::color::Color;

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
