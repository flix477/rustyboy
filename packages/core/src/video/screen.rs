use crate::util::drawer;
use crate::util::drawer::{apply_option_buffer, draw_entity_sprite, DrawnColor, Entity};
use crate::util::wrap_value;
use crate::video::color::{ColorFormat, Color};
use crate::video::memory::background_tile_map::BackgroundTileMap;
use crate::video::memory::sprite_attribute_table::OAMEntry;
use crate::video::tile::Tile;
use crate::video::Video;

pub const SCREEN_SIZE: (usize, usize) = (160, 144);
const BACKGROUND_RELATIVE_SIZE: (u8, u8) = (32, 32);
pub const BACKGROUND_SIZE: (usize, usize) = (256, 256);

#[derive(Default)]
pub struct Screen {
    buffer: Vec<DrawnColor>
}

impl Screen {
    pub fn new() -> Self {
        Self {
            buffer: vec![DrawnColor::default(); SCREEN_SIZE.0 * SCREEN_SIZE.1]
        }
    }

    pub fn draw_line_to_buffer(&mut self, video: &Video) {
        let mut line_buffer = vec![DrawnColor::default(); SCREEN_SIZE.0];
        let ly = video.ly;
        if video.control.lcd_enabled() {
            // Background & Window
            if video.control.bg_window_enabled() {
                line_buffer = Self::draw_background_to_buffer(video, ly);

                if video.control.window_enabled() {
                    let window = Self::draw_window_to_buffer(video, ly);
                    apply_option_buffer(&mut line_buffer, window, false, false);
                }
            }

            if video.control.obj_enabled() {
                // Sprites
                let sprites = Self::draw_sprites_to_buffer(video, ly);
                apply_option_buffer(&mut line_buffer, sprites, true, true);
            }
        }
    }

    fn draw_background_to_buffer(video: &Video, ly: u8) -> Vec<DrawnColor> {
        let (scx, scy) = video.scroll;
        let tile_data = video.vram.tile_data();
        let palette = video.bg_palette();
        let background_tile_map = if video.control.bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        let background_y = scy.wrapping_add(ly);
        let background_tile_map_y = (background_y - background_y % 8) / 8;
        let tile_y = background_y - background_tile_map_y * 8;
        let background_min_x = 0 as usize;
        let background_max_x = SCREEN_SIZE.0 as usize;

        let tiles = background_tile_map.tiles()[background_tile_map_y as usize];
        let addressing_mode = video.control.bg_tile_data_addressing();

        let line: Vec<DrawnColor> = tiles.iter()
            .map(|tile_index| addressing_mode.adjust_index(*tile_index as u16))
            .map(|tile_index| &tile_data[tile_index as usize])
            .flat_map(|tile| tile.colored_line(tile_y, false, false).to_vec())
            .map(|color| {
                DrawnColor {
                    color: palette.color(color),
                    color_value: color
                }
            })
            .collect();

        let mut buffer = vec![DrawnColor::default(); SCREEN_SIZE.0];
        for x in 0..SCREEN_SIZE.0 {
            let background_x = wrap_value(scx as usize + x, BACKGROUND_SIZE.0) as usize;
            buffer[x] = line[background_x];
        }

        buffer
    }

    fn draw_window_to_buffer(video: &Video, ly: u8) -> Vec<Option<DrawnColor>> {
        let (window_x, window_y) = video.window;
        let window_x = window_x.saturating_sub(7);
        if window_y > ly || window_x >= SCREEN_SIZE.0 as u8 { return vec![]; }
        let tile_data = video.vram.tile_data();

        let background_y = ly - window_y;
        let background_tile_map_y = (background_y - background_y % 8) / 8;
        let tile_y = background_y - background_tile_map_y * 8;

        let palette = video.bg_palette();
        let background_tile_map = if video.control.window_bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };
        let tiles = &background_tile_map.tiles()[background_tile_map_y as usize];
        let addressing_mode = video.control.bg_tile_data_addressing();

        tiles.iter()
            .map(|tile_index| addressing_mode.adjust_index(*tile_index as u16))
            .map(|tile_index| &tile_data[tile_index as usize])
            .flat_map(|tile| tile.colored_line(tile_y, false, false).to_vec())
            .map(|color| {
                DrawnColor {
                    color: palette.color(color),
                    color_value: color
                }
            })
            .enumerate()
            .map(|(x, drawn_color)| if x < window_x as usize || x >= SCREEN_SIZE.0 {
                None
            } else {
                Some(drawn_color)
            })
            .collect()
    }

    fn draw_sprites_to_buffer(video: &Video, ly: u8) -> Vec<Option<DrawnColor>> {
        let oam_entries = video.vram.oam().entries();
        let tile_data = video.vram.tile_data();
        let tall_sprites = video.control.obj_big_size();
        let sprite_height = if tall_sprites { 16 } else { 8 };
        let origin = (8, 16);

        struct AdjustedPosition {
            absolute: (u8, u8),
            inner_start: (u8, u8)
        }

        let sprites: Vec<(AdjustedPosition, Vec<u8>)> = oam_entries.iter()
            .map(|entry| {
                let (x, y) = entry.position;

                let start_x = if x < origin.0 { origin.0 - x } else { 0 };
                let start_y = if y < origin.1 { origin.1 - y } else { 0 };

                let position = AdjustedPosition {
                    absolute: (x.saturating_sub(origin.0), y.saturating_sub(origin.1)),
                    inner_start: (start_x, start_y)
                };

                (position, entry)
            })
            .filter(|(position, entry)| {
                // filter out sprites that are not displayed at ly
                let abs_y1 = position.absolute.1;
                let abs_y2 = abs_y1 + (sprite_height - position.inner_start.1);

                entry.visible() && ly >= abs_y1 && ly <= abs_y2
            })
            .map(|(position, entry)| {
                // get the line buffer in the sprite
                let sprite_y = ly - position.absolute.1 + position.inner_start.1;
                let tile_index = (sprite_y - sprite_y % 8) / 8 + entry.tile_number;
                let tile = tile_data[tile_index as usize];
                let tile_y = sprite_y - tile_index * 8;

                let line = tile.colored_line(
                    tile_y,
                    entry.x_flipped(),
                    entry.y_flipped()
                )[position.inner_start.0 as usize..].to_vec();

                (position, line)
            })
            .collect();

//        for sprite in sprites.iter() {
//            let entity = Entity::from_sprite(sprite);
//            let palette = video.obj_palette(sprite.attributes.obj_palette_number());
//            draw_entity_sprite(
//                entity,
//                SCREEN_SIZE,
//                buffer,
//                palette,
//                sprite.attributes.behind_bg(),
//            )
//        }

        vec![]
    }

    pub fn background(video: &Video) -> Vec<DrawnColor> {
        let background_tile_map = if video.control.bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        Self::background_tile_map(video, background_tile_map)
    }

    fn background_tile_map(
        video: &Video,
        background_tile_map: &BackgroundTileMap,
    ) -> Vec<DrawnColor> {
        let tile_data = video.vram.tile_data();
        let mut background_buf =
            vec![DrawnColor::default(); BACKGROUND_SIZE.0 as usize * BACKGROUND_SIZE.1 as usize];
        background_tile_map
            .adjusted_tiles(video.control.bg_tile_data_addressing())
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
                    &video.bg_palette,
                );
            });

        background_buf
    }

    fn window(video: &Video) -> Vec<DrawnColor> {
        let window_tile_map = if video.control.window_bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        Self::background_tile_map(video, window_tile_map)
    }
}

impl Entity {
    pub fn from_sprite(sprite: &Sprite) -> Self {
        Entity {
            width: 8,
            height: sprite.tiles.len() * 8,
            x: sprite.x() as usize,
            y: sprite.y() as usize,
            data: sprite
                .tiles
                .iter()
                .flat_map(|tile| {
                    tile.colored_with_options(sprite.x_flipped(), sprite.y_flipped())
                        .to_vec()
                })
                .collect(),
        }
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
    pub tiles: Vec<Tile>,
    pub attributes: OAMEntry,
}

impl Sprite {
    pub fn x(&self) -> u8 {
        self.attributes.position.0
    }

    pub fn y(&self) -> u8 {
        self.attributes.position.1
    }

    pub fn x_flipped(&self) -> bool {
        self.attributes.x_flipped()
    }

    pub fn y_flipped(&self) -> bool {
        self.attributes.y_flipped()
    }
}
