mod color;
mod tile;
use crate::bus::{Readable, Writable};
use crate::video::tile::Tile;

pub struct Video {
    enabled: bool,
    scroll: (u8, u8),
    window: (u8, u8),
    video_memory: VideoMemory
}

impl Video {
    pub fn new() -> Video {
        Video {
            enabled: true,
            scroll: (0, 0),
            window: (0, 0),
            video_memory: VideoMemory::new()
        }
    }

    fn control(&self) -> u8 {
        unimplemented!()
    }
}

impl Readable for Video {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x8000...0x9FFF => self.video_memory.read(address), // video ram
            0xFF40 => self.control(), // lcdc control
            0xFF41 => unimplemented!(), // lcdc status
            0xFF42 => self.scroll.1, // lcdc scroll y
            0xFF43 => self.scroll.0, // lcdc scroll x
            0xFF44 => unimplemented!(), // lcdc LY
            0xFF45 => unimplemented!(), // lcdc LYC
            0xFF46 => unimplemented!(), // dma transfer
            0xFF47 => unimplemented!(), // background & window palette
            0xFF48 => unimplemented!(), // object palette 0
            0xFF49 => unimplemented!(), // object palette 1
            0xFF4A => self.window.1, // window y position
            0xFF4B => self.window.0, // window x position
            _ => 0
        }
    }
}

impl Writable for Video {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x8000...0x9FFF => self.video_memory.write(address, value), // video ram
            0xFF40 => unimplemented!(), // lcdc control
            0xFF41 => unimplemented!(), // lcdc status
            0xFF42 => self.scroll.1 = value, // lcdc scroll y
            0xFF43 => self.scroll.0 = value, // lcdc scroll x
            0xFF44 => unimplemented!(), // lcdc LY
            0xFF45 => unimplemented!(), // lcdc LYC
            0xFF46 => unimplemented!(), // dma transfer
            0xFF47 => unimplemented!(), // background & window palette
            0xFF48 => unimplemented!(), // object palette 0
            0xFF49 => unimplemented!(), // object palette 1
            0xFF4A => self.window.1 = value, // window y position
            0xFF4B => self.window.0 = value, // window x position
            _ => {}
        }
    }
}

struct VideoMemory {
    tile_data: [Tile; 384]
}

impl VideoMemory {
    pub fn new() -> VideoMemory {
        VideoMemory {
            tile_data: [Tile::new([0; 8]); 384]
        }
    }

    pub fn tile_line_at(&self, address: u16) -> u8 {
        let tile_address = 0x8000u16.saturating_sub(address);
        let tile_base_address = (tile_address - tile_address % 16) / 16;
        let line_idx = ((tile_address - tile_address % 2) - tile_base_address) / 2;
        let line = self.tile_data[tile_base_address as usize].line(line_idx as u8);
        (line >> ((tile_address % 2) * 8)) as u8
    }
}

impl Readable for VideoMemory {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x8000...0x97FF => self.tile_line_at(address),
            _ => 0
        }
    }
}

impl Writable for VideoMemory {
    fn write(&mut self, address: u16, _value: u8) {
        match address {
            _ => {}
        }
    }
}