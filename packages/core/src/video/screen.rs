use crate::util::drawer::{apply_option_buffer, DrawnColor};
use crate::util::wrap_value;
use crate::video::color::ColorFormat;
use crate::video::memory::sprite_attribute_table::OAMEntry;
use crate::video::tile::Tile;
use crate::video::palette::Palette;
use crate::video::memory::VideoMemory;
use crate::video::control_register::ControlRegister;

pub const SCREEN_SIZE: (usize, usize) = (160, 144);
pub const BACKGROUND_SIZE: (usize, usize) = (256, 256);

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
    buffer: Vec<DrawnColor>
}

impl Screen {
    pub fn buffer(&self, format: ColorFormat) -> Vec<u8> {
        self.buffer.iter()
            .flat_map(|drawn_color| drawn_color.color.format(format))
            .collect()
    }

    pub fn draw_line_to_buffer(&mut self, video: VideoInformation, ly: u8) {
        let line = Self::draw_line(video, ly);
        let base_buffer_index = ly as usize * SCREEN_SIZE.0;

        for (x, color) in line.iter().enumerate() {
            self.buffer[base_buffer_index + x] = *color;
        }
    }

    fn draw_line(video: VideoInformation, ly: u8) -> Vec<DrawnColor> {
        let mut line_buffer = vec![DrawnColor::default(); SCREEN_SIZE.0];
        if video.control.lcd_enabled() {
            // Background & Window
            if video.control.bg_window_enabled() {
                line_buffer = Self::draw_background_line(&video, ly);

                if video.control.window_enabled() {
                    let window = Self::draw_window_line(&video, ly);
                    apply_option_buffer(&mut line_buffer, window, false);
                }
            }

            if video.control.obj_enabled() {
                // Sprites
                let sprites = Self::draw_sprites_line(&video, ly);
                apply_option_buffer(&mut line_buffer, sprites, true);
            }
        }

        line_buffer
    }

    fn draw_background_line(video: &VideoInformation, ly: u8) -> Vec<DrawnColor> {
        let (scx, scy) = video.scroll;
        let tile_data = video.vram.tile_data();
        let palette = video.bg_palette;
        let background_tile_map = if video.control.bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };

        let background_y = scy.wrapping_add(ly);
        let background_tile_map_y = (background_y - background_y % 8) / 8;
        let tile_y = background_y - background_tile_map_y * 8;

        let tiles = background_tile_map.tiles()[background_tile_map_y as usize];
        let addressing_mode = video.control.bg_tile_data_addressing();

        let line: Vec<DrawnColor> = tiles.iter()
            .map(|tile_index| addressing_mode.adjust_index(*tile_index as u16))
            .map(|tile_index| &tile_data[tile_index as usize])
            .flat_map(|tile| tile.colored_line(tile_y, false, false).to_vec())
            .map(|color| {
                DrawnColor {
                    color: palette.color(color),
                    color_value: color,
                    low_priority: false
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

    fn draw_window_line(video: &VideoInformation, ly: u8) -> Vec<Option<DrawnColor>> {
        let (window_x, window_y) = video.window;
        let window_x = window_x.saturating_sub(7);
        if window_y > ly || window_x >= SCREEN_SIZE.0 as u8 { return vec![]; }
        let tile_data = video.vram.tile_data();

        let background_y = ly - window_y;
        let background_tile_map_y = (background_y - background_y % 8) / 8;
        let tile_y = background_y - background_tile_map_y * 8;

        let palette = video.bg_palette;
        let background_tile_map = if video.control.window_bg_map() == 0 {
            &video.vram.background_tile_maps().0
        } else {
            &video.vram.background_tile_maps().1
        };
        let tiles = &background_tile_map.tiles()[background_tile_map_y as usize];
        let addressing_mode = video.control.bg_tile_data_addressing();

        let line: Vec<DrawnColor> = tiles.iter()
            .map(|tile_index| addressing_mode.adjust_index(*tile_index as u16))
            .map(|tile_index| &tile_data[tile_index as usize])
            .flat_map(|tile| tile.colored_line(tile_y, false, false).to_vec())
            .map(|color| {
                DrawnColor {
                    color: palette.color(color),
                    color_value: color,
                    low_priority: false
                }
            })
            .collect();

        let mut buffer = vec![None; SCREEN_SIZE.0];
        for buffer_x in window_x as usize..SCREEN_SIZE.0 {
            let x = buffer_x - window_x as usize;
            buffer[buffer_x] = Some(line[x]);
        }
        buffer
    }

    fn draw_sprites_line(video: &VideoInformation, ly: u8) -> Vec<Option<DrawnColor>> {
        let oam_entries = video.vram.oam().entries();
        let tile_data = video.vram.tile_data();
        let tall_sprites = video.control.obj_big_size();
        let sprite_height = if tall_sprites { 16 } else { 8 };
        let origin = (8, 16);

        struct AdjustedPosition {
            absolute: (u8, u8),
            inner_start: (u8, u8),
            inner_end_x: u8
        }

        let sprites: Vec<(AdjustedPosition, Vec<DrawnColor>)> = oam_entries.iter()
            .filter(|entry| entry.visible(tall_sprites))
            .map(|entry| {
                let (x, y) = entry.position;

                let start_x = if x < origin.0 { origin.0 - x } else { 0 };
                let start_y = if y < origin.1 { origin.1 - y } else { 0 };
                let absolute = (x.saturating_sub(origin.0), y.saturating_sub(origin.1));

                let inner_end_x = if 8 - start_x + absolute.0 >= SCREEN_SIZE.0 as u8 {
                    8 - (8 - start_x + absolute.0 - SCREEN_SIZE.0 as u8)
                } else { 8 };

                let position = AdjustedPosition {
                    absolute,
                    inner_start: (start_x, start_y),
                    inner_end_x
                };

                (position, entry)
            })
            .filter(|(position, _)| {
                // filter out sprites that are not displayed at ly
                let abs_y1 = position.absolute.1;
                let abs_y2 = abs_y1 + sprite_height - position.inner_start.1;

                ly >= abs_y1 && ly < abs_y2
            })
            .map(|(position, entry)| {
                // get the line buffer in the sprite
                let sprite_y = ly - position.absolute.1 + position.inner_start.1;
                let tile_relative_index = (sprite_y - sprite_y % 8) / 8;
                let tile_index = tile_relative_index + entry.tile_number;
                let tile = tile_data[tile_index as usize];
                let tile_y = sprite_y - tile_relative_index * 8;
                let palette = video.obj_palette(entry.obj_palette_number());

                let line: Vec<DrawnColor> = tile.colored_line(
                    tile_y,
                    entry.x_flipped(),
                    entry.y_flipped()
                )[position.inner_start.0 as usize..position.inner_end_x as usize].iter()
                    .map(|color| DrawnColor {
                        color: palette.color(*color),
                        color_value: *color,
                        low_priority: entry.behind_bg()
                    })
                    .collect();

                (position, line)
            })
            .collect();

        let mut buffer = vec![None; SCREEN_SIZE.0];
        for (position, line) in &sprites {
            for (x, color) in line.iter().enumerate() {
                buffer[position.absolute.0 as usize + x] = Some(*color);
            }
        }

        buffer
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            buffer: vec![DrawnColor::default(); SCREEN_SIZE.0 * SCREEN_SIZE.1]
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
