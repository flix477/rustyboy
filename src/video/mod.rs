pub mod color;
pub mod tile;
mod screen;
mod palette;
mod register;
mod status_register;
mod control_register;
mod memory;
use crate::bus::{Readable, Writable};
use crate::video::palette::Palette;
use self::register::Register;
use self::status_register::{StatusRegister, StatusMode};
use self::control_register::ControlRegister;
use self::memory::VideoMemory;
use self::screen::Screen;

pub struct Video {
    control: ControlRegister,
    status: StatusRegister,
    mode: StatusMode,
    scroll: (u8, u8),
    window: (u8, u8),
    ly: u8,
    lyc: u8,
    bg_palette: Palette,
    obj_palette0: Palette,
    obj_palette1: Palette,
    // TODO: CGB color palettes
    vram: VideoMemory,
    screen: Screen
}

impl Video {
    pub fn new() -> Video {
        Video {
            control: ControlRegister::new(),
            status: StatusRegister::new(), // TODO: is it tho
            mode: StatusMode::HBlank,
            scroll: (0, 0),
            window: (0, 0),
            ly: 0,
            lyc: 0,
            bg_palette: Palette::new(),
            obj_palette0: Palette::new(),
            obj_palette1: Palette::new(),
            vram: VideoMemory::new(),
            screen: Screen::new()
        }
    }

    pub fn memory(&self) -> &VideoMemory { &self.vram }
    pub fn screen(&self) -> &Screen { &self.screen }
    pub fn obj_palette0(&self) -> &Palette { &self.obj_palette0 }
    pub fn obj_palette1(&self) -> &Palette { &self.obj_palette1 }
}

impl Readable for Video {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFE00...0xFE9F |
            0x9800...0x9FFF => self.vram.read(address),
            0x8000...0x97FF => {
                let addressing_mode = self.control.bg_tile_data_addressing();
                self.vram.read(addressing_mode.adjust_address(address))
            }, // video ram
            0xFF40 => self.control.get(), // lcdc control
            0xFF41 => self.status.generate(&self), // lcdc status
            0xFF42 => self.scroll.1, // lcdc scroll y
            0xFF43 => self.scroll.0, // lcdc scroll x
            0xFF44 => self.ly, // lcdc LY
            0xFF45 => self.lyc, // lcdc LYC
            0xFF47 => self.bg_palette.get(), // background & window palette
            0xFF48 => self.obj_palette0.get(), // object palette 0
            0xFF49 => self.obj_palette1.get(), // object palette 1
            0xFF4A => self.window.1, // window y position
            0xFF4B => self.window.0, // window x position
            _ => unimplemented!()
        }
    }
}

impl Writable for Video {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFE00...0xFE9F |
            0x9800...0x9FFF => self.vram.write(address, value),
            0x8000...0x97FF => {
                let addressing_mode = self.control.bg_tile_data_addressing();
                self.vram.write(addressing_mode.adjust_address(address), value)
            }, // video ram
            0xFF40 => self.control.set(value), // lcdc control
            0xFF41 => self.status.set(value), // lcdc status
            0xFF42 => self.scroll.1 = value, // lcdc scroll y
            0xFF43 => self.scroll.0 = value, // lcdc scroll x
            0xFF44 => self.ly = 0, // reset lcdc LY
            0xFF45 => self.lyc = value, // lcdc LYC
            0xFF47 => self.bg_palette.set(value), // background & window palette
            0xFF48 => self.obj_palette0.set(value), // object palette 0
            0xFF49 => self.obj_palette1.set(value), // object palette 1
            0xFF4A => self.window.1 = value, // window y position
            0xFF4B => self.window.0 = value, // window x position
            _ => unimplemented!()
        }
    }
}