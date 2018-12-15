use crate::video::color::Color;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    data: [u16; 8]
}

impl Tile {
    pub fn new(data: [u16; 8]) -> Tile {
        Tile { data }
    }

    pub fn line(&self, line: u8) -> u16 {
        self.data[line as usize]
    }

    pub fn set_line(&mut self, line: u8, value: u16) {
        self.data[line as usize] = value;
    }

    pub fn colored(&self) -> [Color; 64] {
        let mut colors: [Color; 64] = [Color::Black; 64];
        for row in 0..8 {
            for col in 0..8 {
                colors[row * 8 + col] = Color::from(((self.data[row] as u8).wrapping_shr(2 * col as u32)) & 0b11);
            }
        }
        colors
    }
}