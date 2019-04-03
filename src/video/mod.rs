pub mod color;
mod control_register;
mod memory;
mod palette;
mod register;
mod screen;
pub mod status_register;
pub mod tile;

use self::control_register::ControlRegister;
use self::memory::VideoMemory;
use self::register::Register;
use self::screen::Screen;
use self::status_register::{StatusMode, StatusRegister};
use crate::bus::{Readable, Writable};
use crate::processor::interrupt::{Interrupt, InterruptHandler};
use crate::video::memory::background_tile_map::BackgroundTileMap;
use crate::video::palette::Palette;
use crate::video::tile::Tile;

const CLOCK_FREQUENCY: f64 = 4194304.0; // Hz

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
    screen: Screen,
    cycles_left: u16,
}

impl Video {
    pub fn new() -> Video {
        let mut bg_palette = Palette::new();
        bg_palette.set(0xFC);

        let mut obj_palette0 = Palette::new();
        obj_palette0.set(0xFF);

        let mut obj_palette1 = Palette::new();
        obj_palette1.set(0xFF);

        Video {
            control: ControlRegister::new(),
            status: StatusRegister::new(), // TODO: is it tho
            mode: StatusMode::ReadingOAM,
            scroll: (0, 0),
            window: (0, 0),
            ly: 0,
            lyc: 0,
            bg_palette,
            obj_palette0,
            obj_palette1,
            vram: VideoMemory::new(),
            screen: Screen::new(),
            cycles_left: 0,
        }
    }

    pub fn memory(&self) -> &VideoMemory {
        &self.vram
    }
    pub fn mode(&self) -> StatusMode {
        self.mode
    }
    pub fn screen(&self) -> &Screen {
        &self.screen
    }
    pub fn obj_palette0(&self) -> &Palette {
        &self.obj_palette0
    }
    pub fn obj_palette1(&self) -> &Palette {
        &self.obj_palette1
    }

    pub fn clock(&mut self, interrupt_handler: &mut InterruptHandler, cycles: u8) {
        self.cycles_left = self.cycles_left.saturating_sub(cycles as u16);
        if self.cycles_left == 0 {
            self.step(interrupt_handler);
            self.cycles_left = self.mode_cycle_length();
        }
    }

    fn step(&mut self, interrupt_handler: &mut InterruptHandler) {
        if self.mode == StatusMode::ReadingOAM {
            self.mode = StatusMode::LCDTransfer;
            self.render_scanline();
        } else if self.mode == StatusMode::LCDTransfer {
            self.ly += 1;
            if self.check_lyc(interrupt_handler) {
                interrupt_handler.request_interrupt(Interrupt::LCDCStat);
            }
            self.mode = StatusMode::HBlank;
            if self.status.hblank_interrupt_enabled() {
                interrupt_handler.request_interrupt(Interrupt::LCDCStat);
            }
        } else if self.mode == StatusMode::VBlank
            || (self.mode == StatusMode::HBlank && self.ly < 144)
        {
            self.mode = StatusMode::ReadingOAM;
            if self.status.oam_interrupt_enabled() {
                interrupt_handler.request_interrupt(Interrupt::LCDCStat);
            }
        } else {
            self.ly = 0;
            self.mode = StatusMode::VBlank;
            interrupt_handler.request_interrupt(Interrupt::VBlank);
            if self.status.vblank_interrupt_enabled() {
                interrupt_handler.request_interrupt(Interrupt::LCDCStat);
            }
        }
    }

    fn mode_cycle_length(&self) -> u16 {
        match self.mode {
            StatusMode::ReadingOAM => 80,
            StatusMode::VBlank => 4560,
            StatusMode::LCDTransfer => {
                let mut length = 172;
                // TODO: accurate timing
                length
            }
            StatusMode::HBlank => {
                // TODO: accurate timing
                204
            }
        }
    }

    fn check_lyc(&self, interrupt_handler: &mut InterruptHandler) -> bool {
        self.status.lyc_interrupt_enabled() && self.ly == self.lyc
    }

    fn render_scanline(&mut self) {
        //        const MAX: u8 = 160;
        //        let y = self.ly;
        //        let tile_data = self.vram.tile_data();
        //        let (scroll_x, scroll_y) = self.scroll;
        //        println!("{} + {}", scroll_y, y);
        //        let (rel_x, rel_y) = ((scroll_x + 8) as u16, (scroll_y + y) as u16);
        //
        //        let mut line: Vec<u8> = vec![0; 160];
        //
        //        if self.control.bg_window_enabled() {
        //            // 1: background
        //            let background = if self.control.bg_map() == 0 {
        //                &self.vram.background_tile_maps().0
        //            } else {
        //                &self.vram.background_tile_maps().1
        //            };
        //
        //            let bg_line = self.line_from_bg_map(rel_x, rel_y, background, tile_data);
        //
        //            // 2: window
        //            if self.control.window_enabled() &&
        //                self.window.1 <= rel_y &&
        //                self.window.0 > 6 && self.window.0 < 166
        //            {
        //                let window = if self.control.window_bg_map() == 0 {
        //                    &self.vram.background_tile_maps().0
        //                } else {
        //                    &self.vram.background_tile_maps().1
        //                };
        //
        //                let window_line = self.line_from_bg_map(rel_x, rel_y, window, tile_data);
        //            }
        //        }
        //
        //        // 3: sprites
    }

    fn line_from_bg_map(
        &self,
        rel_x: u8,
        rel_y: u8,
        bg_map: &BackgroundTileMap,
        tile_data: &[Tile; 384],
    ) -> Vec<u8> {
        let first_tile_x = ((rel_x - rel_x % 8) / 8) as usize;
        // TODO: the last_tile thing might be optimised
        let last_tile_x = ((rel_x + 160 - (rel_x + 160) % 8) / 8) as usize;
        let tile_y = ((rel_y - rel_y % 8) / 8) as usize;
        bg_map.tiles()[tile_y][first_tile_x..=last_tile_x]
            .iter()
            .map(|tile_id| tile_data[*tile_id as usize])
            .flat_map(|tile| tile.formatted_line(rel_y).to_vec())
            .collect::<Vec<u8>>()
    }
}

impl Readable for Video {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFE00...0xFE9F => {
                if self.mode != StatusMode::LCDTransfer && self.mode != StatusMode::ReadingOAM {
                    self.vram.read(address)
                } else {
                    0xFF
                }
            } // oam
            0x9800...0x9FFF | 0x8000...0x97FF => {
                if self.mode != StatusMode::LCDTransfer {
                    let mut address = address;
                    if 0x8000 <= address && 0x97FF >= address {
                        let addressing_mode = self.control.bg_tile_data_addressing();
                        address = addressing_mode.adjust_address(address);
                    }
                    self.vram.read(address)
                } else {
                    0xFF
                }
            } // video ram
            0xFF40 => self.control.get(),          // lcdc control
            0xFF41 => self.status.generate(&self), // lcdc status
            0xFF42 => self.scroll.1,               // lcdc scroll y
            0xFF43 => self.scroll.0,               // lcdc scroll x
            0xFF44 => self.ly,                     // lcdc LY
            0xFF45 => self.lyc,                    // lcdc LYC
            0xFF47 => self.bg_palette.get(),       // background & window palette
            0xFF48 => self.obj_palette0.get(),     // object palette 0
            0xFF49 => self.obj_palette1.get(),     // object palette 1
            0xFF4A => self.window.1,               // window y position
            0xFF4B => self.window.0,               // window x position
            _ => unimplemented!(),
        }
    }
}

impl Writable for Video {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFE00...0xFE9F => {
                if self.mode != StatusMode::LCDTransfer && self.mode != StatusMode::ReadingOAM {
                    self.vram.write(address, value);
                }
            } // oam
            0x9800...0x9FFF | 0x8000...0x97FF => {
                if self.mode != StatusMode::LCDTransfer {
                    let mut address = address;
                    if 0x8000 <= address && 0x97FF >= address {
                        let addressing_mode = self.control.bg_tile_data_addressing();
                        address = addressing_mode.adjust_address(address);
                    }
                    self.vram.write(address, value);
                }
            } // video ram
            0xFF40 => self.control.set(value),      // lcdc control
            0xFF41 => self.status.set(value),       // lcdc status
            0xFF42 => self.scroll.1 = value,        // lcdc scroll y
            0xFF43 => self.scroll.0 = value,        // lcdc scroll x
            0xFF44 => self.ly = 0,                  // reset lcdc LY
            0xFF45 => self.lyc = value,             // lcdc LYC
            0xFF47 => self.bg_palette.set(value),   // background & window palette
            0xFF48 => self.obj_palette0.set(value), // object palette 0
            0xFF49 => self.obj_palette1.set(value), // object palette 1
            0xFF4A => self.window.1 = value,        // window y position
            0xFF4B => self.window.0 = value,        // window x position
            _ => unimplemented!(),
        }
    }
}
