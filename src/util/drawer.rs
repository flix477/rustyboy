use crate::video::color::Color;

pub struct Entity {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub data: [Color; 64],
}

pub fn draw_entity(entity: Entity, dimensions: (usize, usize), buf: &mut Vec<Color>) {
    for entity_y in 0..entity.height {
        let y = entity_y + entity.y;
        let base_idx = y * dimensions.0;
        let entity_base_idx = entity_y * entity.width;
        for x in 0..entity.width {
            let buf_idx = base_idx + entity.x + x;
            let entity_idx = entity_base_idx + x;
            buf[buf_idx] = entity.data[entity_idx];
        }
    }
}
