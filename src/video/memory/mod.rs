pub mod sprite_attribute_table;
pub mod background_tile_map;
use crate::bus::{Readable, Writable};
use crate::video::tile::Tile;
use self::sprite_attribute_table::SpriteAttributeTable;
use self::background_tile_map::BackgroundTileMap;

pub struct VideoMemory {
    tile_data: [Tile; 384],
    oam: SpriteAttributeTable,
    background_tile_maps: (BackgroundTileMap, BackgroundTileMap)
}

impl VideoMemory {
    pub fn new() -> VideoMemory {
        VideoMemory {
            tile_data: [Tile::new([0; 8]); 384],
            oam: SpriteAttributeTable::new(),
            background_tile_maps: (BackgroundTileMap::new(), BackgroundTileMap::new())
        }
    }

    pub fn tile_data(&self) -> &[Tile; 384] { &self.tile_data }
    pub fn oam(&self) -> &SpriteAttributeTable { &self.oam }
    pub fn background_tile_maps(&self) -> &(BackgroundTileMap, BackgroundTileMap) { &self.background_tile_maps }

    fn tile_idx_at(&self, address: u16) -> (u16, u16, u8) {
        let tile_address = address.saturating_sub(0x8000);
        let tile_base_address = (tile_address - tile_address % 16) / 16;
        let line_idx = ((tile_address - tile_address % 2) - tile_base_address * 16) / 2;
        (tile_address, tile_base_address, line_idx as u8)
    }

    fn tile_line_at(&self, address: u16) -> u8 {
        let (tile_address, tile_idx, line_idx) = self.tile_idx_at(address);
        let line = self.tile_data[tile_idx as usize].line(line_idx);
        (line >> ((tile_address % 2) * 8)) as u8
    }

    fn set_tile_line_at(&mut self, address: u16, value: u8) {
        let (tile_address, tile_idx, line_idx) = self.tile_idx_at(address);
        println!("yo tile: {}", tile_idx);
        let value = (value as u16).wrapping_shl((!(tile_address % 2).saturating_mul(8)) as u32);
        self.tile_data[tile_idx as usize].set_line(line_idx, value);
    }
}

impl Readable for VideoMemory {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x8000...0x97FF => self.tile_line_at(address),
            0x9800...0x9BFF => self.background_tile_maps.0.tile_idx_at(address - 0x9800),
            0x9C00...0x9FFF => self.background_tile_maps.1.tile_idx_at(address - 0x9C00),
            0xFE00...0xFE9F => self.oam.read(address),
            _ => unimplemented!()
        }
    }
}

impl Writable for VideoMemory {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x8000...0x97FF => self.set_tile_line_at(address, value),
            0x9800...0x9BFF => self.background_tile_maps.0.set_tile_idx_at(address - 0x9800, value),
            0x9C00...0x9FFF => self.background_tile_maps.1.set_tile_idx_at(address - 0x9C00, value),
            0xFE00...0xFE9F => self.oam.write(address, value),
            _ => unimplemented!()
        }
    }
}