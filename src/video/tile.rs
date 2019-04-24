use crate::video::color::Color;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    data: [u16; 8],
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
                colors[row * 8 + col] = self.color_at(col as u8, row as u8);
            }
        }
        colors
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        ((self.data[y as usize] as u8).wrapping_shr(2 * x as u32)) & 0b11
    }

    pub fn color_at(&self, x: u8, y: u8) -> Color {
        let x = 7 - x;
        let line = self.data[y as usize];
        let (msb, lsb) = ((line >> 8) as u8, line as u8);
        let value = (((lsb >> x) & 1) << 1) | ((msb >> x) & 1);
        Color::from(value)
    }

    pub fn formatted_line(&self, y: u8) -> [u8; 8] {
        let mut colors: [u8; 8] = [0; 8];
        for col in 0..8 {
            colors[(y * 8 + col) as usize] =
                ((self.data[y as usize] as u8).wrapping_shr(2 * col as u32)) & 0b11;
        }
        colors
    }
}

#[test]
fn colored() {
    let tile_data = [0, 6168, 14392, 6168, 6168, 6168, 15420, 0];
    let tile = Tile::new(tile_data);
    let colored = tile.colored();
    let expected = [
        Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White,
        Color::White, Color::White, Color::White, Color::Black, Color::Black, Color::White, Color::White, Color::White,
        Color::White, Color::White, Color::Black, Color::Black, Color::Black, Color::White, Color::White, Color::White,
        Color::White, Color::White, Color::White, Color::Black, Color::Black, Color::White, Color::White, Color::White,
        Color::White, Color::White, Color::White, Color::Black, Color::Black, Color::White, Color::White, Color::White,
        Color::White, Color::White, Color::White, Color::Black, Color::Black, Color::White, Color::White, Color::White,
        Color::White, Color::White, Color::Black, Color::Black, Color::Black, Color::Black, Color::White, Color::White,
        Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White,
    ];

    assert_eq!(colored.to_vec(), expected.to_vec());
}