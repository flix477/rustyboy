use crate::util::drawer;
use crate::util::drawer::Entity;
use crate::util::wrap_value;
use crate::video::color::{Color, ColorFormat};
use crate::video::memory::sprite_attribute_table::OAMEntry;
use crate::video::tile::Tile;
use crate::video::Video;

pub const SCREEN_SIZE: (usize, usize) = (160, 144);
const BACKGROUND_RELATIVE_SIZE: (u8, u8) = (32, 32);
const BACKGROUND_SIZE: (usize, usize) = (256, 256);

pub struct Screen {}

impl Screen {
    pub fn draw(video: &Video) -> Vec<u8> {
        Self::draw_with_options(video, ColorFormat::RGB)
    }

    pub fn draw_with_options(video: &Video, format: ColorFormat) -> Vec<u8> {
        let mut buf = vec![Color::White; SCREEN_SIZE.0 * SCREEN_SIZE.1];
        let oam_entries = video.vram.oam().entries();
        let tile_data = video.vram.tile_data();

        // Background
        if video.control.bg_window_enabled() {
            let (scx, scy) = video.scroll;
            let background_buf = Self::background(video);
            for y in 0..SCREEN_SIZE.1 {
                let background_y = wrap_value(scy + y as u8, SCREEN_SIZE.1 as u8) as usize;
                for x in 0..SCREEN_SIZE.0 {
                    let background_x = wrap_value(scx + x as u8, SCREEN_SIZE.0 as u8) as usize;
                    let idx = y * SCREEN_SIZE.0 + x as usize;
                    let background_idx = background_y * BACKGROUND_SIZE.0 + background_x;
                    buf[idx as usize] = background_buf[background_idx];
                }
            }
        }

        // Sprites
        let sprites: Vec<Sprite> = if video.control.obj_enabled() {
            oam_entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.visible())
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
            Self::draw_entity(entity, &mut buf);
        }

        buf.iter()
            .flat_map(|color| color.format(format))
            .collect::<Vec<u8>>()
    }

    fn draw_entity(entity: Entity, buf: &mut Vec<Color>) {
        drawer::draw_entity(
            entity,
            SCREEN_SIZE,
            buf,
        );
    }

    pub fn background(video: &Video) -> Vec<Color> {
        let tile_data = video.vram.tile_data();

        let background_tile_map = if video.control.bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        let mut background_buf =
            vec![Color::White; BACKGROUND_SIZE.0 as usize * BACKGROUND_SIZE.1 as usize];
        background_tile_map
            .tiles()
            .iter()
            .map(|tile_index| &tile_data[*tile_index as usize])
            .enumerate()
            .for_each(|(index, tile)| {
                let relative_y = (index - index % BACKGROUND_RELATIVE_SIZE.0 as usize)
                    / BACKGROUND_RELATIVE_SIZE.0 as usize;
                let relative_x = index - relative_y * BACKGROUND_RELATIVE_SIZE.0 as usize;
                let (x, y) = (8 * relative_x, 8 * relative_y);
                let entity = Entity::from_tile(tile, x, y);

                drawer::draw_entity(
                    entity,
                    (BACKGROUND_SIZE.0 as usize, BACKGROUND_SIZE.1 as usize),
                    &mut background_buf,
                );
            });

        background_buf
    }
}

impl Entity {
    pub fn from_sprite(sprite: &Sprite) -> Self {
        Self::from_tile(
            &sprite.tile,
            sprite.x() as usize - 8,
            sprite.y() as usize - 16,
        )
    }

    pub fn from_tile(tile: &Tile, x: usize, y: usize) -> Self {
        Entity {
            width: 8,
            height: 8,
            x,
            y,
            data: tile.colored().to_vec(),
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
