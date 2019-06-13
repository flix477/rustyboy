use crate::video::control_register::TileDataAddressing;

#[derive(Default)]
pub struct BackgroundTileMap {
    tiles: [[u8; 32]; 32],
}

impl BackgroundTileMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn adjusted_tiles(&self, addressing_mode: TileDataAddressing) -> Vec<u16> {
        self.tiles
            .iter()
            .flat_map(|row| row)
            .cloned()
            .map(|tile_index| {
                let tile_index = u16::from(tile_index);
                if addressing_mode == TileDataAddressing::Mode8800 && tile_index < 128 {
                    tile_index + 256
                } else {
                    tile_index
                }
            })
            .collect()
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
}
