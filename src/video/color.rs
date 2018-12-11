#[derive(Copy, Clone)]
pub enum Color {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Invalid value.")
        }
    }
}