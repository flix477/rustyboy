use crate::util::savestate::{read_savestate_byte, LoadSavestateError, Savestate};

#[derive(Default, Clone)]
pub struct BackgroundTileMap {
    tiles: [[u8; 32]; 32],
}

impl BackgroundTileMap {
    pub fn new() -> Self {
        Self::default()
    }

    fn tile_info_at(&self, address: u16) -> (usize, usize) {
        let row = (address - (address % 32)) / 32;
        let column = address - row * 32;
        (row as usize, column as usize)
    }

    pub fn tile_idx_at(&self, address: u16) -> u8 {
        let (row, column) = self.tile_info_at(address);
        self.tiles[row][column]
    }

    pub fn set_tile_idx_at(&mut self, address: u16, value: u8) {
        let (row, column) = self.tile_info_at(address);
        self.tiles[row][column] = value;
    }

    pub fn tiles(&self) -> &[[u8; 32]; 32] {
        &self.tiles
    }
}

impl Savestate for BackgroundTileMap {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        for y in &self.tiles {
            for x in y {
                buffer.push(*x);
            }
        }
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<'a, u8>,
    ) -> Result<(), LoadSavestateError> {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                self.tiles[y][x] = read_savestate_byte(buffer)?;
            }
        }

        Ok(())
    }
}
