use crate::video::register::Register;
use crate::util::bits::get_bit;

pub struct ControlRegister {
    register: u8
}

// TODO: bunch of stuff

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister {
            register: 0
        }
    }

    pub fn lcd_enabled(&self) -> bool {
        get_bit(self.register, 7)
    }

//    pub fn window_tilemap(&self) -> bool {
//        get_bit(self.register, 6)
//    }

    pub fn window_enabled(&self) -> bool {
        get_bit(self.register, 5)
    }

    pub fn bg_tile_data_addressing(&self) -> TileDataAddressing {
        if get_bit(self.register, 4) {
            TileDataAddressing::Mode8000
        } else {
            TileDataAddressing::Mode8800
        }
    }

//    pub fn bg_tilemap(&self) -> bool {
//        get_bit(self.register, 3)
//    }

//    pub fn obj_size(&self) -> bool {
//        get_bit(self.register, 2)
//    }

//    pub fn obj_enabled(&self) -> bool {
//        get_bit(self.register, 1)
//    }

//    pub fn layer_priority(&self) -> bool {
//        get_bit(self.register, 0)
//    }
}

impl Register for ControlRegister {
    fn get(&self) -> u8 { self.register }
    fn set(&mut self, value: u8) { self.register = value; }
}

pub enum TileDataAddressing {
    Mode8000,
    Mode8800
}

impl TileDataAddressing {
    pub fn adjust_address(&self, address: u16) -> u16 {
        if let TileDataAddressing::Mode8000 = self {
            address
        } else {
            // TODO: could be written single line with math
            match address {
                0x8000...0x87FF => address + 0x1000,
                0x9000...0x97FF => address - 0x1000,
                _ => address
            }
        }
    }
}