use crate::util::drawer;
use crate::util::drawer::Entity;
use crate::video::color::Color;
use crate::video::memory::background_tile_map::BackgroundTileMap;
use crate::video::memory::sprite_attribute_table::OAMEntry;
use crate::video::tile::Tile;
use crate::video::Video;

pub struct Screen {
    pub dimensions: (u8, u8),
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            dimensions: (160, 144),
        }
    }

    pub fn draw(&self, video: &Video) -> Vec<u8> {
        let mut buf = vec![Color::White; self.dimensions.0 as usize * self.dimensions.1 as usize];
        let oam_entries = video.vram.oam().entries();
        let tile_data = video.vram.tile_data();

        // Background
        if video.control.bg_window_enabled() {
            println!("enabled");
            let background_tile_map = if video.control.bg_map() == 0 {
                &video.vram.background_tile_maps().0
            } else {
                &video.vram.background_tile_maps().1
            };

            let origin = video.scroll;
            let tiles = background_tile_map.tiles().iter()
                .map(|tile_index| tile_data[*tile_index as usize]);

        }

        // Sprites
        let sprites: Vec<Sprite> = if video.control.obj_enabled() {
            oam_entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.attributes != 0)
                .map(|(id, entry)| Sprite {
                    id: id as u8,
                    tile: tile_data[entry.tile_number as usize],
                    attributes: *entry,
                })
                .collect()
        } else {
            vec![]
        };

        for sprite in sprites.iter() {
            let entity = Entity::from_sprite(sprite);
            self.draw_entity(entity, &mut buf);
        }

        buf.iter()
            .flat_map(|color| color.to_rgb().to_vec())
            .collect::<Vec<u8>>()
    }

    fn draw_entity(&self, entity: Entity, buf: &mut Vec<Color>) {
        drawer::draw_entity(
            entity,
            (self.dimensions.0 as usize, self.dimensions.1 as usize),
            buf,
        );
    }
}

impl Entity {
    pub fn from_sprite(sprite: &Sprite) -> Self {
        Entity {
            width: 8,
            height: 8,
            x: sprite.x(),
            y: sprite.y(),
            data: sprite.tile.colored(),
        }
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub id: u8,
    pub tile: Tile,
    pub attributes: OAMEntry,
}

impl Sprite {
    pub fn x(&self) -> u8 {
        self.attributes.position.0
    }
    pub fn y(&self) -> u8 {
        self.attributes.position.1
    }
}
