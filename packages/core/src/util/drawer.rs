use crate::video::color::Color;
use crate::video::palette::Palette;

pub struct Entity {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub data: Vec<u8>,
}

#[derive(Default, Copy, Clone)]
pub struct DrawnColor {
    pub color: Color,
    pub color_value: u8
}

pub fn draw_entity(entity: Entity, dimensions: (usize, usize), buf: &mut Vec<DrawnColor>, palette: &Palette) {
    draw_entity_with_options(entity, dimensions, buf, palette, false, false);
}

pub fn draw_entity_with_options(
    entity: Entity,
    dimensions: (usize, usize),
    buf: &mut Vec<DrawnColor>,
    palette: &Palette,
    transparency: bool,
    prefer_existing: bool,
) {
    for entity_y in 0..entity.height {
        let y = entity_y + entity.y;
        if y >= dimensions.1 {
            continue;
        }
        let base_idx = y * dimensions.0;
        let entity_base_idx = entity_y * entity.width;
        for x in 0..entity.width {
            let buf_idx = base_idx + entity.x + x;
            let entity_idx = entity_base_idx + x;
            let buffer_color = buf[buf_idx].color_value;
            let entity_color = entity.data[entity_idx];
            if (!transparency || entity_color != 0)
                && (!prefer_existing || buffer_color == 0)
            {
                buf[buf_idx] = DrawnColor{
                    color_value: entity_color,
                    color: palette.color(entity_color)
                }
            }
        }
    }
}
