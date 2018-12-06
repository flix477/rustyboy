pub enum Color {
    Dark,
    Gray,
    Light,
    Clear
}

impl Color {
    pub fn from_value(value: u8) -> Color {
        match value {
            1 => Color::Dark,
            2 => Color::Gray,
            3 => Color::Light,
            _ => Color::Clear
        }
    }
}