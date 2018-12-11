use super::color::Color;

pub struct Palette {
    palette: [Color; 4],
    register: u8
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            palette: [
                Color::White,
                Color::LightGray,
                Color::DarkGray,
                Color::Black
            ],
            register: 0b00011011
        }
    }

    pub fn color(&self, idx: u8) -> Color {
        Color::from((self.register >> (2 * idx)) & 0b11)
    }

    pub fn get(&self) -> u8 {
        self.register
    }

    pub fn set(&mut self, value: u8) {
        self.register = value;
    }
}