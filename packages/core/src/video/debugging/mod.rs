use crate::video::color::{Color, ColorFormat};
use crate::video::screen::{Screen, VideoInformation, BACKGROUND_SIZE};
use crate::video::memory::VideoMemory;
use crate::video::control_register::ControlRegister;
use crate::video::palette::Palette;

// The difference between this struct and VideoInformation is that
// this one does not borrow, everything is copied.
// This makes it easier to pass around for debugging.
#[derive(Clone)]
pub struct VideoDebugInformation {
    pub scroll: (u8, u8),
    pub window: (u8, u8),
    pub vram: VideoMemory,
    pub control: ControlRegister,
    pub bg_palette: Palette,
    pub obj_palette0: Palette,
    pub obj_palette1: Palette,
}

impl VideoDebugInformation {
    fn into(&self) -> VideoInformation<'_> {
        VideoInformation {
            scroll: self.scroll,
            window: self.window,
            vram: &self.vram,
            control: &self.control,
            bg_palette: &self.bg_palette,
            obj_palette0: &self.obj_palette0,
            obj_palette1: &self.obj_palette1
        }
    }
}

pub fn background_map_buffer(
    background_map_index: u8,
    debug_info: &VideoDebugInformation,
    format: ColorFormat,
) -> Vec<u8> {
    let background_map = if background_map_index == 0 {
        &debug_info.vram.background_tile_maps().0
    } else {
        &debug_info.vram.background_tile_maps().1
    };

    (0..BACKGROUND_SIZE.1)
        .flat_map(|line| Screen::draw_background_map_line(&debug_info.into(), background_map, line))
        .flat_map(|drawn_color| drawn_color.color.format(format))
        .collect()
}

pub fn tile_buffer(
    tile_index: usize,
    debug_info: &VideoDebugInformation,
    format: ColorFormat,
) -> Vec<u8> {
    let tile = debug_info.vram.tile_data()[tile_index];
    tile.colored()
        .iter()
        .flat_map(|color_value| Color::from(*color_value).format(format))
        .collect()
}

pub fn sprite_buffer(
    sprite_index: usize,
    debug_info: &VideoDebugInformation,
    format: ColorFormat,
) -> Vec<u8> {
    let tile_data = debug_info.vram.tile_data();
    let sprite = debug_info.vram.oam().entries()[sprite_index];
    let tiles_count = if debug_info.control.obj_big_size() {
        2
    } else {
        1
    };

    (0..tiles_count)
        .flat_map(|tile_index| {
            tile_data[tile_index + sprite.tile_number as usize]
                .colored()
                .to_vec()
        })
        .flat_map(|color| Color::from(color).format(format))
        .collect()
}
