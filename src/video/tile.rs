#[derive(Copy, Clone)]
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
}