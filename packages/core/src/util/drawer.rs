use crate::video::color::Color;

pub struct Entity {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub data: Vec<Color>,
}

pub fn draw_entity(entity: Entity, dimensions: (usize, usize), buf: &mut Vec<Color>) {
    draw_entity_with_options(entity, dimensions, buf, false, false);
}

pub fn draw_entity_with_options(
    entity: Entity,
    dimensions: (usize, usize),
    buf: &mut Vec<Color>,
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
            let current_color = buf[buf_idx];
            let color = entity.data[entity_idx];
            if (!transparency || color != Color::White)
                && (!prefer_existing || current_color == Color::White)
            {
                buf[buf_idx] = entity.data[entity_idx];
            }
        }
    }
}
