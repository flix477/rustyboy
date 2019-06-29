use crate::util::drawer::{apply_option_buffer, DrawnColor};
use crate::util::wrap_value;
use crate::video::color::ColorFormat;
use crate::video::control_register::ControlRegister;
use crate::video::memory::background_tile_map::BackgroundTileMap;
use crate::video::memory::sprite_attribute_table::OAMEntry;
use crate::video::memory::VideoMemory;
use crate::video::palette::Palette;
use crate::video::tile::Tile;

pub const SCREEN_SIZE: (usize, usize) = (160, 144);
pub const BACKGROUND_SIZE: (usize, usize) = (256, 256);
const TILE_SIZE: u8 = 8;
const SPRITES_ORIGIN: (u8, u8) = (8, 16);

pub struct VideoInformation<'a> {
    pub scroll: (u8, u8),
    pub window: (u8, u8),
    pub vram: &'a VideoMemory,
    pub control: &'a ControlRegister,
    pub bg_palette: &'a Palette,
    pub obj_palette0: &'a Palette,
    pub obj_palette1: &'a Palette,
}

impl<'a> VideoInformation<'a> {
    pub fn obj_palette(&self, number: u8) -> &Palette {
        if number == 0 {
            &self.obj_palette0
        } else {
            &self.obj_palette1
        }
    }
}

pub struct Screen {
    buffer: Vec<DrawnColor>,
}

impl Screen {
    pub fn buffer(&self, format: ColorFormat) -> Vec<u8> {
        self.buffer
            .iter()
            .flat_map(|drawn_color| drawn_color.color.format(format))
            .collect()
    }

    pub fn draw_line_to_buffer(&mut self, video: VideoInformation<'_>, ly: u8) {
        let line = Self::draw_line(video, ly);
        let base_buffer_index = ly as usize * SCREEN_SIZE.0;

        for (x, color) in line.iter().enumerate() {
            self.buffer[base_buffer_index + x] = *color;
        }
    }

    fn draw_line(video: VideoInformation<'_>, ly: u8) -> Vec<DrawnColor> {
        let mut line_buffer = vec![DrawnColor::default(); SCREEN_SIZE.0];
        if video.control.lcd_enabled() {
            // Background & Window
            if video.control.bg_window_enabled() {
                line_buffer = Self::draw_background_line(&video, ly);

                if video.control.window_enabled() {
                    let window = Self::draw_window_line(&video, ly);
                    apply_option_buffer(&mut line_buffer, window, false, false);
                }
            }

            if video.control.obj_enabled() {
                // Sprites
                let sprites = Self::draw_sprites_line(&video, ly);
                apply_option_buffer(&mut line_buffer, sprites, true, true);
            }
        }

        line_buffer
    }

    fn draw_background_line(video: &VideoInformation<'_>, ly: u8) -> Vec<DrawnColor> {
        let (scx, scy) = video.scroll;
        let background_tile_map = if video.control.bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        let background_y = scy.wrapping_add(ly);

        let line =
            Self::draw_background_map_line(video, background_tile_map, background_y as usize);

        (0..SCREEN_SIZE.0)
            .map(|x| line[wrap_value(scx as usize + x, BACKGROUND_SIZE.0) as usize])
            .collect()
    }

    fn draw_window_line(video: &VideoInformation<'_>, ly: u8) -> Vec<Option<DrawnColor>> {
        let (window_x, window_y) = video.window;
        let window_x = window_x.saturating_sub(7) as usize;
        if window_y > ly || window_x >= SCREEN_SIZE.0 {
            return vec![];
        }

        let background_y = ly - window_y;

        let background_tile_map = if video.control.window_bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        let line =
            Self::draw_background_map_line(video, background_tile_map, background_y as usize);

        (0..SCREEN_SIZE.0)
            .map(|x| {
                if x >= window_x {
                    Some(line[x - window_x])
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn draw_background_map_line(
        video: &VideoInformation<'_>,
        background_tile_map: &BackgroundTileMap,
        background_y: usize,
    ) -> Vec<DrawnColor> {
        let tile_data = video.vram.tile_data();
        let background_tile_map_y =
            (background_y - background_y % TILE_SIZE as usize) / TILE_SIZE as usize;
        let tile_y = background_y - background_tile_map_y * TILE_SIZE as usize;

        let palette = video.bg_palette;
        let tiles = &background_tile_map.tiles()[background_tile_map_y as usize];
        let addressing_mode = video.control.bg_tile_data_addressing();

        tiles
            .iter()
            .map(|tile_index| addressing_mode.adjust_index(u16::from(*tile_index)))
            .map(|tile_index| &tile_data[tile_index as usize])
            .flat_map(|tile| tile.colored_line(tile_y as u8, false, false).to_vec())
            .map(|color| DrawnColor {
                color: palette.color(color),
                color_value: color,
                low_priority: false,
            })
            .collect()
    }

    fn draw_sprites_line(video: &VideoInformation<'_>, ly: u8) -> Vec<Option<DrawnColor>> {
        let oam_entries = video.vram.oam().entries();
        let tile_data = video.vram.tile_data();
        let tall_sprites = video.control.obj_big_size();
        let sprite_height = if tall_sprites { 16 } else { 8 };

        struct AdjustedPosition {
            absolute: (u8, u8),
            inner_start: (u8, u8),
        }

        let sprites: Vec<(u8, Vec<DrawnColor>)> = oam_entries
            .iter()
            .filter(|entry| entry.visible(tall_sprites))
            .map(|entry| {
                let (x, y) = entry.position;
                let (origin_x, origin_y) = SPRITES_ORIGIN;

                let start_x = if x < origin_x { origin_x - x } else { 0 };
                let start_y = if y < origin_y { origin_y - y } else { 0 };

                let absolute = (x.saturating_sub(origin_x), y.saturating_sub(origin_y));

                let position = AdjustedPosition {
                    absolute,
                    inner_start: (start_x, start_y),
                };

                (position, entry)
            })
            .filter(|(position, _)| {
                // filter out sprites that are not displayed at ly
                let abs_y1 = position.absolute.1;
                let abs_y2 = abs_y1 + sprite_height - position.inner_start.1;

                ly >= abs_y1 && ly < abs_y2
            })
            .take(10) // display only the first 10 sprites of the line
            .map(|(position, entry)| {
                // get the line buffer in the sprite
                let palette = video.obj_palette(entry.obj_palette_number());

                let sprite_y = ly - position.absolute.1 + position.inner_start.1;

                let tile_relative_index = (sprite_y - sprite_y % TILE_SIZE) / TILE_SIZE;
                let tile = tile_data[(tile_relative_index + entry.tile_number) as usize];
                let tile_y = sprite_y - tile_relative_index * TILE_SIZE;

                let abs_x2 = TILE_SIZE - position.inner_start.0 + position.absolute.0;
                let inner_end_x = if abs_x2 >= SCREEN_SIZE.0 as u8 {
                    TILE_SIZE - (abs_x2 - SCREEN_SIZE.0 as u8)
                } else {
                    TILE_SIZE
                };

                let line: Vec<DrawnColor> =
                    tile.colored_line(tile_y, entry.x_flipped(), entry.y_flipped())
                        [position.inner_start.0 as usize..inner_end_x as usize]
                        .iter()
                        .map(|color| DrawnColor {
                            color: palette.color(*color),
                            color_value: *color,
                            low_priority: entry.behind_bg(),
                        })
                        .collect();

                (position.absolute.0, line)
            })
            .collect();

        let mut buffer = vec![None; SCREEN_SIZE.0];
        // loop in reverse so we draw sprites that are earlier in oam as higher priority
        for (absolute_x, line) in sprites.iter().rev() {
            for (x, color) in line.iter().enumerate() {
                let index = *absolute_x as usize + x;
                if color.color_value != 0 {
                    buffer[index] = Some(*color);
                }
            }
        }

        buffer
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            buffer: vec![DrawnColor::default(); SCREEN_SIZE.0 * SCREEN_SIZE.1],
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
