use crate::video::color::Color;

pub struct Entity {
    pub width: u8,
    pub height: u8,
    pub x: u8,
    pub y: u8,
    pub data: [Color; 64],
}

pub fn draw_entity(entity: Entity, dimensions: (usize, usize), buf: &mut Vec<Color>) {
    for entity_y in 0..entity.height {
        let y = entity_y + entity.y;
        let base_idx = y as u16 * dimensions.0 as u16;
        let entity_base_idx = entity_y * entity.width;
        for x in 0..entity.width {
            let buf_idx = base_idx + entity.x as u16 + x as u16;
            let entity_idx = entity_base_idx + x;
            buf[buf_idx as usize] = entity.data[entity_idx as usize];
        }
    }
}
