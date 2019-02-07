use crate::bus::{Readable, Writable};
use crate::util::bits::get_bit;

pub struct SpriteAttributeTable {
    table: [OAMEntry; 40],
}

impl SpriteAttributeTable {
    pub fn new() -> Self {
        SpriteAttributeTable {
            table: [OAMEntry::new(); 40],
        }
    }

    fn entry_byte_at(&self, address: u16) -> (u8, u8) {
        let address = address - 0xFE00;
        let entry_idx = (address - address % 4) / 4;
        let byte_idx = address - entry_idx * 4;
        (entry_idx as u8, byte_idx as u8)
    }

    pub fn entries(&self) -> &[OAMEntry; 40] {
        &self.table
    }
}

impl Readable for SpriteAttributeTable {
    fn read(&self, address: u16) -> u8 {
        let (entry_idx, byte_idx) = self.entry_byte_at(address);
        self.table[entry_idx as usize].byte(byte_idx as u8)
    }
}

impl Writable for SpriteAttributeTable {
    fn write(&mut self, address: u16, value: u8) {
        let (entry_idx, byte_idx) = self.entry_byte_at(address);
        self.table[entry_idx as usize].set_byte(byte_idx as u8, value);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OAMEntry {
    pub position: (u8, u8),
    pub tile_number: u8,
    pub attributes: u8,
}

impl OAMEntry {
    pub fn new() -> Self {
        OAMEntry {
            position: (0, 0),
            tile_number: 0,
            attributes: 0,
        }
    }

    pub fn byte(&self, byte: u8) -> u8 {
        match byte {
            0 => self.position.1,
            1 => self.position.0,
            2 => self.tile_number,
            3 => self.attributes,
            _ => panic!("Invalid byte."),
        }
    }

    pub fn set_byte(&mut self, byte: u8, value: u8) {
        match byte {
            0 => self.position.1 = value,
            1 => self.position.0 = value,
            2 => self.tile_number = value,
            3 => self.attributes = value,
            _ => panic!("Invalid byte."),
        }
    }

    pub fn obj_palette_number(&self) -> u8 {
        (self.attributes & 0b10000) >> 4
    }

    pub fn x_flipped(&self) -> bool {
        get_bit(self.attributes, 5)
    }

    pub fn y_flipped(&self) -> bool {
        get_bit(self.attributes, 6)
    }

    pub fn tile_vram_bank(&self) -> u8 {
        (self.attributes & 0b1000) >> 3
    }

    pub fn cgb_palette_number(&self) -> u8 {
        self.attributes & 0b11
    }

    pub fn priority(&self) -> u8 {
        (self.attributes & 0b10000000) >> 7
    }
}
