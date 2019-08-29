use crate::util::savestate::{
    read_savestate_u16, write_savestate_u16, LoadSavestateError, Savestate,
};

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

    pub fn colored(&self) -> [u8; 64] {
        self.colored_with_options(false, false)
    }

    pub fn colored_with_options(&self, x_flipped: bool, y_flipped: bool) -> [u8; 64] {
        let mut colors: [u8; 64] = [0; 64];
        for row in 0..8 {
            for col in 0..8 {
                let x = if x_flipped { 7 - col } else { col };
                let y = if y_flipped { 7 - row } else { row };
                colors[row * 8 + col] = self.color_value_at(x as u8, y as u8);
            }
        }
        colors
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        ((self.data[y as usize] as u8).wrapping_shr(2 * u32::from(x))) & 0b11
    }

    pub fn color_value_at(&self, x: u8, y: u8) -> u8 {
        let x = 7 - x;
        let line = self.data[y as usize];
        let (msb, lsb) = ((line >> 8) as u8, line as u8);
        (((lsb >> x) & 1) << 1) | ((msb >> x) & 1)
    }

    pub fn colored_line(&self, y: u8, x_flipped: bool, y_flipped: bool) -> [u8; 8] {
        let mut line = [0; 8];
        line.iter_mut().enumerate().for_each(|(col, line_color)| {
            let x = if x_flipped { 7 - col } else { col };
            let y = if y_flipped { 7 - y } else { y };
            *line_color = self.color_value_at(x as u8, y as u8)
        });

        line
    }
}

impl Savestate for Tile {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        for line in &self.data {
            write_savestate_u16(buffer, *line);
        }
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<'a, u8>,
    ) -> Result<(), LoadSavestateError> {
        for line in 0..self.data.len() {
            self.data[line] = read_savestate_u16(buffer)?;
        }

        Ok(())
    }
}

#[test]
fn colored() {
    use crate::video::color::Color;

    let tile_data = [0, 6168, 14392, 6168, 6168, 6168, 15420, 0];
    let tile = Tile::new(tile_data);
    let colored = tile
        .colored()
        .iter()
        .map(|color_value| Color::from(*color_value))
        .collect::<Vec<Color>>();

    let expected = [
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::Black,
        Color::Black,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::Black,
        Color::Black,
        Color::Black,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::Black,
        Color::Black,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::Black,
        Color::Black,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::Black,
        Color::Black,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::Black,
        Color::Black,
        Color::Black,
        Color::Black,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
        Color::White,
    ];

    assert_eq!(colored, expected.to_vec());
}
