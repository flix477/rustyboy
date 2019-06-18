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
    pub color_value: u8,
}

pub fn draw_entity(
    entity: Entity,
    dimensions: (usize, usize),
    buf: &mut Vec<DrawnColor>,
    palette: &Palette,
) {
    draw_entity_with_options(entity, dimensions, buf, palette, false, false, (0, 0));
}

pub fn draw_entity_sprite(
    entity: Entity,
    dimensions: (usize, usize),
    buf: &mut Vec<DrawnColor>,
    palette: &Palette,
    prefer_existing: bool,
) {
    draw_entity_with_options(
        entity,
        dimensions,
        buf,
        palette,
        true,
        prefer_existing,
        (8, 16),
    );
}

pub fn draw_entity_with_options(
    entity: Entity,
    dimensions: (usize, usize),
    buffer: &mut Vec<DrawnColor>,
    palette: &Palette,
    transparency: bool,
    prefer_existing: bool,
    origin: (usize, usize),
) {
    let absolute_entity_y = entity.y.saturating_sub(origin.1); // 0
    let absolute_entity_x = entity.x.saturating_sub(origin.0);

    let starting_y = if entity.y < origin.1 {
        origin.1 - entity.y
    } else {
        0
    }; // 4

    let starting_x = if entity.x < origin.0 {
        origin.0 - entity.x
    } else {
        0
    };

    for entity_y in starting_y..entity.height {
        let buffer_y = entity_y + absolute_entity_y - starting_y; // 0 + 0 -
        if buffer_y >= dimensions.1 {
            break;
        }

        let base_buffer_idx = buffer_y * dimensions.0;
        let entity_base_idx = entity_y * entity.width;

        for entity_x in starting_x..entity.width {
            let buffer_x = absolute_entity_x + entity_x - starting_x;
            if buffer_x >= dimensions.0 {
                break;
            }

            let buffer_idx = base_buffer_idx + buffer_x;
            let entity_idx = entity_base_idx + entity_x;
            let buffer_color = buffer[buffer_idx].color_value;
            let entity_color = entity.data[entity_idx];

            if (!transparency || entity_color != 0) && (!prefer_existing || buffer_color == 0) {
                buffer[buffer_idx] = DrawnColor {
                    color_value: entity_color,
                    color: palette.color(entity_color),
                }
            }
        }
    }
}

pub fn apply_option_buffer(
    buffer: &mut Vec<DrawnColor>,
    option_buffer: Vec<Option<DrawnColor>>,
    transparency: bool,
    prefer_existing: bool
) {
    for (index, option) in option_buffer.iter().enumerate() {
        if let Some(drawn_color) = option {
            let buffer_color = buffer[index].color_value;
            if (!transparency || drawn_color.color_value != 0) && (!prefer_existing || buffer_color == 0) {
                buffer[index] = *drawn_color;
            }
        }
    }
}