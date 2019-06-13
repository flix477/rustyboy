#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl Color {
    pub fn format(self, format: ColorFormat) -> Vec<u8> {
        match format {
            ColorFormat::RGB => self.to_rgb().to_vec(),
            ColorFormat::RGBA => self.to_rgba().to_vec(),
        }
    }

    pub fn to_rgb(self) -> [u8; 3] {
        match self {
            Color::White => [255, 255, 255],
            Color::LightGray => [170, 170, 170],
            Color::DarkGray => [85, 85, 85],
            Color::Black => [0, 0, 0],
        }
    }

    pub fn to_rgba(self) -> [u8; 4] {
        match self {
            Color::White => [255, 255, 255, 255],
            Color::LightGray => [170, 170, 170, 255],
            Color::DarkGray => [85, 85, 85, 255],
            Color::Black => [0, 0, 0, 255],
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Invalid value."),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ColorFormat {
    RGB,
    RGBA,
}
