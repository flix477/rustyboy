use crate::video::Video;
use crate::video::tile::Tile;
use crate::video::memory::sprite_attribute_table::OAMEntry;
use crate::video::memory::background_tile_map::BackgroundTileMap;

pub struct Screen {
    pub dimensions: (u8, u8)
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            dimensions: (160, 144)
        }
    }

    pub fn draw(&self, video: &Video) -> Vec<u8> {
        let oam_entries = video.vram.oam().entries();
        let tile_data = video.vram.tile_data();
        let sprites: Vec<Sprite> = if video.control.obj_enabled() {
            oam_entries.iter()
                .enumerate()
                .filter(|(id, entry)| {
                    entry.attributes != 0
                })
                .map(|(id, entry)| {
                    Sprite {
                        id: id as u8,
                        tile: tile_data[entry.tile_number as usize],
                        attributes: *entry
                    }
                })
                .collect()
        } else { vec![] };

        for sprite in sprites.iter() {
            println!("{:?}", sprite);
        }

        if video.control.bg_window_enabled() {
            if video.control.window_enabled() {
                let bg_map = if video.control.window_bg_map() == 0 {
                    &video.vram.background_tile_maps().0
                } else {
                    &video.vram.background_tile_maps().1
                };

                let tiles: Vec<Tile> = Self::resolve_tiles(bg_map, tile_data);
                println!("{:?}", tiles);
            }

            let bg_map = if video.control.bg_map() == 0 {
                &video.vram.background_tile_maps().0
            } else {
                &video.vram.background_tile_maps().1
            };

            let tiles: Vec<Tile> = Self::resolve_tiles(bg_map, tile_data);
        }

        vec![0; self.dimensions.0 as usize * self.dimensions.1 as usize * 3]
    }

    pub fn resolve_tiles(bg_map: &BackgroundTileMap, tile_data: &[Tile; 384]) -> Vec<Tile> {
        bg_map.tiles().iter()
            .flat_map(|row| row.iter()
                .map(|tile_idx| tile_data[*tile_idx as usize]))
            .collect()
    }
}

#[derive(Debug)]
struct Sprite {
    pub id: u8,
    pub tile: Tile,
    pub attributes: OAMEntry
}

impl Sprite {
    pub fn x(&self) -> u8 { self.attributes.position.0 }
    pub fn y(&self) -> u8 { self.attributes.position.1 }
}