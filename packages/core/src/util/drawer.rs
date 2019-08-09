use crate::video::color::Color;

#[derive(Default, Copy, Clone)]
pub struct DrawnColor {
    pub color: Color,
    pub color_value: u8,
    pub low_priority: bool,
}

pub fn apply_option_buffer(
    buffer: &mut [DrawnColor],
    option_buffer: &[Option<DrawnColor>],
    transparency: bool,
    prefer_existing: bool,
) {
    buffer
        .iter_mut()
        .zip(option_buffer.iter())
        .for_each(|(buffer_color, drawn_color)| {
            if let Some(drawn_color) = drawn_color {
                let prefer_existing = prefer_existing && drawn_color.low_priority;
                if (!transparency || drawn_color.color_value != 0)
                    && (!prefer_existing || buffer_color.color_value == 0)
                {
                    *buffer_color = *drawn_color;
                }
            }
        });
}
