//use crate::video::color::{Color, ColorFormat};
//use crate::video::screen::{Screen, VideoInformation, BACKGROUND_SIZE};
//
//pub fn background_map_buffer(
//    background_map_index: u8,
//    video_information: &VideoInformation<'_>,
//    format: ColorFormat,
//) -> Vec<u8> {
//    let background_map = if background_map_index == 0 {
//        &video_information.vram.background_tile_maps().0
//    } else {
//        &video_information.vram.background_tile_maps().1
//    };
//
//    (0..BACKGROUND_SIZE.1)
//        .flat_map(|line| Screen::draw_background_map_line(video_information, background_map, line))
//        .flat_map(|drawn_color| drawn_color.color.format(format))
//        .collect()
//}
//
//pub fn tile_buffer(
//    tile_index: usize,
//    video_information: &VideoInformation<'_>,
//    format: ColorFormat,
//) -> Vec<u8> {
//    let tile = video_information.vram.tile_data()[tile_index];
//    tile.colored()
//        .iter()
//        .flat_map(|color_value| Color::from(*color_value).format(format))
//        .collect()
//}
//
//pub fn sprite_buffer(
//    sprite_index: usize,
//    video_information: &VideoInformation<'_>,
//    format: ColorFormat,
//) -> Vec<u8> {
//    let tile_data = video_information.vram.tile_data();
//    let sprite = video_information.vram.oam().entries()[sprite_index];
//    let tiles_count = if video_information.control.obj_big_size() {
//        2
//    } else {
//        1
//    };
//
//    (0..tiles_count)
//        .flat_map(|tile_index| {
//            tile_data[tile_index + sprite.tile_number as usize]
//                .colored()
//                .to_vec()
//        })
//        .flat_map(|color| Color::from(color).format(format))
//        .collect()
//}
