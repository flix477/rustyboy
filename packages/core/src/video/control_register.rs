use crate::util::bits::get_bit;

#[derive(Copy, Clone)]
pub struct ControlRegister {
    register: u8,
}

// TODO: bunch of stuff

impl ControlRegister {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lcd_enabled(self) -> bool {
        get_bit(self.register, 7)
    }

    // which background map the window uses for rendering
    pub fn window_bg_map(self) -> u8 {
        get_bit(self.register, 6) as u8
    }

    // whether the window shall be displayed or not
    pub fn window_enabled(self) -> bool {
        get_bit(self.register, 5)
    }

    // which addressing mode the background and window use to pick tiles
    pub fn bg_tile_data_addressing(self) -> TileDataAddressing {
        if get_bit(self.register, 4) {
            TileDataAddressing::Mode8000
        } else {
            TileDataAddressing::Mode8800
        }
    }

    // which background map the background uses for rendering
    pub fn bg_map(self) -> u8 {
        get_bit(self.register, 3) as u8
    }

    // controls the sprite size (false = 1 tile, true = 2 stacked vertically)
    pub fn obj_big_size(self) -> bool {
        get_bit(self.register, 2)
    }

    // whether sprites are displayed or not
    pub fn obj_enabled(self) -> bool {
        get_bit(self.register, 1)
    }

    // when false, both background and window become blank, regardless of window_enabled
    pub fn bg_window_enabled(self) -> bool {
        get_bit(self.register, 0)
    }

    pub fn get(self) -> u8 {
        self.register
    }

    pub fn set(&mut self, value: u8) {
        self.register = value;
    }
}

impl Default for ControlRegister {
    fn default() -> Self {
        ControlRegister { register: 0x91 }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TileDataAddressing {
    Mode8000,
    Mode8800,
}

impl TileDataAddressing {
    pub fn adjust_index(self, tile_index: u16) -> u16 {
        if self == TileDataAddressing::Mode8800 && tile_index < 128 {
            tile_index + 256
        } else {
            tile_index
        }
    }
}
