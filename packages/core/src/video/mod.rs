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
use crate::video::palette::Palette;

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
            status: StatusRegister::new(),
            mode: StatusMode::ReadingOAM,
            scroll: (0, 0),
            window: (0, 0),
            ly: 144,
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

    pub fn clock(&mut self, interrupt_handler: &mut InterruptHandler) -> bool {
        if self.mode == StatusMode::VBlank {
            self.set_ly(143 + (10 - self.cycles_left / 456) as u8, interrupt_handler)
        }

        self.cycles_left = self.cycles_left.saturating_sub(1);
        if self.cycles_left == 0 {
            let vblank = self.step(interrupt_handler);
            self.cycles_left = self.mode_cycle_length();
            vblank
        } else {
            false
        }
    }

    fn step(&mut self, interrupt_handler: &mut InterruptHandler) -> bool {
        if self.mode == StatusMode::ReadingOAM {
            self.mode = StatusMode::LCDTransfer;
        } else if self.mode == StatusMode::LCDTransfer {
            self.set_ly(self.ly + 1, interrupt_handler);
            self.set_mode(StatusMode::HBlank, interrupt_handler)
        } else if self.mode == StatusMode::HBlank {
            if self.ly < 143 {
                self.set_mode(StatusMode::ReadingOAM, interrupt_handler);
            } else {
                self.set_mode(StatusMode::VBlank, interrupt_handler);
            }
        } else {
            self.set_ly(0, interrupt_handler);
            interrupt_handler.request_interrupt(Interrupt::VBlank);
            self.set_mode(StatusMode::ReadingOAM, interrupt_handler);
            return true;
        }

        false
    }

    fn set_mode(&mut self, mode: StatusMode, interrupt_handler: &mut InterruptHandler) {
        self.mode = mode;
        match mode {
            StatusMode::ReadingOAM => {
                if self.status.oam_interrupt_enabled() {
                    interrupt_handler.request_interrupt(Interrupt::LCDCStat);
                }
            }
            StatusMode::HBlank => {
                if self.status.hblank_interrupt_enabled() {
                    interrupt_handler.request_interrupt(Interrupt::LCDCStat);
                }
            }
            StatusMode::VBlank => {
                if self.status.vblank_interrupt_enabled() {
                    interrupt_handler.request_interrupt(Interrupt::LCDCStat);
                }
            }
            _ => {}
        };
    }

    fn set_ly(&mut self, value: u8, interrupt_handler: &mut InterruptHandler) {
        self.ly = value;
        if self.check_lyc() {
            interrupt_handler.request_interrupt(Interrupt::LCDCStat)
        }
    }

    fn mode_cycle_length(&self) -> u16 {
        match self.mode {
            StatusMode::ReadingOAM => 80,
            StatusMode::VBlank => 4560,
            StatusMode::LCDTransfer => {
                // TODO: accurate timing
                172
            }
            StatusMode::HBlank => {
                // TODO: accurate timing
                204
            }
        }
    }

    fn check_lyc(&self) -> bool {
        self.status.lyc_interrupt_enabled() && self.ly == self.lyc
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
                //                if self.mode != StatusMode::LCDTransfer && self.mode != StatusMode::ReadingOAM {
                self.vram.write(address, value);
                //                }
            } // oam
            0x9800...0x9FFF | 0x8000...0x97FF => {
                //                if self.mode != StatusMode::LCDTransfer {
                //                    if 0x8000 <= address && 0x97FF >= address {
                //                        let addressing_mode = self.control.bg_tile_data_addressing();
                //                        address = addressing_mode.adjust_address(address);
                //                    }
                self.vram.write(address, value);
                //                }
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
