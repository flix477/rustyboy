use crate::video::color::Color;

#[derive(Default, Copy, Clone)]
pub struct DrawnColor {
    pub color: Color,
    pub color_value: u8,
    pub low_priority: bool,
}

pub fn apply_option_buffer(
    buffer: &mut Vec<DrawnColor>,
    option_buffer: Vec<Option<DrawnColor>>,
    transparency: bool,
) {
    for (index, option) in option_buffer.iter().enumerate() {
        if let Some(drawn_color) = option {
            let buffer_color = buffer[index].color_value;
            let prefer_existing = drawn_color.low_priority;
            if (!transparency || drawn_color.color_value != 0)
                && (!prefer_existing || buffer_color == 0)
            {
                buffer[index] = *drawn_color;
            }
        }
    }
}
