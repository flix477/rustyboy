pub mod color;
mod control_register;
pub mod debugging;
mod memory;
pub mod palette;
mod position_registers;
pub mod screen;
pub mod status_register;
pub mod tile;

use self::control_register::ControlRegister;
use self::memory::VideoMemory;
use self::position_registers::PositionRegisters;
use self::status_register::{StatusMode, StatusRegister};
use crate::bus::{Readable, Writable};
use crate::processor::interrupt::{Interrupt, InterruptHandler};
use crate::util::savestate::{
    read_savestate_byte, read_savestate_u16, write_savestate_u16, LoadSavestateError, Savestate,
};
use crate::video::debugging::VideoDebugInformation;
use crate::video::palette::Palette;
use crate::video::screen::{Screen, VideoInformation};

pub struct Video {
    control: ControlRegister,
    status: StatusRegister,
    mode: StatusMode,
    position_registers: PositionRegisters,
    bg_palette: Palette,
    obj_palette0: Palette,
    obj_palette1: Palette,
    // TODO: CGB color palettes
    vram: VideoMemory,
    cycles_left: u16,
    screen: Screen,
}

impl Video {
    pub fn new() -> Video {
        Self::default()
    }

    pub fn memory(&self) -> &VideoMemory {
        &self.vram
    }
    pub fn mode(&self) -> StatusMode {
        self.mode
    }
    pub fn obj_palette0(&self) -> &Palette {
        &self.obj_palette0
    }
    pub fn obj_palette1(&self) -> &Palette {
        &self.obj_palette1
    }
    pub fn bg_palette(&self) -> &Palette {
        &self.bg_palette
    }
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn obj_palette(&self, number: u8) -> &Palette {
        if number == 0 {
            &self.obj_palette0
        } else {
            &self.obj_palette1
        }
    }

    pub fn clock(&mut self, interrupt_handler: &mut InterruptHandler) -> Option<StatusMode> {
        if self.mode == StatusMode::VBlank {
            self.set_ly(143 + (10 - self.cycles_left / 456) as u8, interrupt_handler)
        }

        self.cycles_left = self.cycles_left.saturating_sub(1);
        if self.cycles_left == 0 {
            self.step(interrupt_handler);
            self.cycles_left = self.mode_cycle_length();

            if self.mode == StatusMode::HBlank {
                let video = VideoInformation {
                    scroll: self.position_registers.scroll(),
                    window: self.position_registers.window(),
                    vram: &self.vram,
                    control: &self.control,
                    bg_palette: &self.bg_palette,
                    obj_palette0: &self.obj_palette0,
                    obj_palette1: &self.obj_palette1,
                };
                self.screen
                    .draw_line_to_buffer(video, self.position_registers.ly());
            }

            self.position_registers.on_mode_change(self.mode);

            Some(self.mode)
        } else {
            None
        }
    }

    fn step(&mut self, interrupt_handler: &mut InterruptHandler) {
        if self.mode == StatusMode::ReadingOAM {
            self.mode = StatusMode::LCDTransfer;
        } else if self.mode == StatusMode::LCDTransfer {
            self.set_mode(StatusMode::HBlank, interrupt_handler)
        } else if self.mode == StatusMode::HBlank {
            self.set_ly(self.position_registers.ly() + 1, interrupt_handler);
            if self.position_registers.ly() < 144 {
                self.set_mode(StatusMode::ReadingOAM, interrupt_handler);
            } else {
                self.set_mode(StatusMode::VBlank, interrupt_handler);
                interrupt_handler.request_interrupt(Interrupt::VBlank);
            }
        } else {
            self.set_ly(0, interrupt_handler);
            self.set_mode(StatusMode::ReadingOAM, interrupt_handler);
        }
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
        self.position_registers.set_ly(value, self.mode);
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
        self.status.lyc_interrupt_enabled()
            && self.position_registers.ly() == self.position_registers.lyc()
    }

    pub fn video_information(&self) -> VideoInformation<'_> {
        VideoInformation {
            scroll: self.position_registers.scroll(),
            window: self.position_registers.window(),
            vram: &self.vram,
            control: &self.control,
            bg_palette: &self.bg_palette,
            obj_palette0: &self.obj_palette0,
            obj_palette1: &self.obj_palette1,
        }
    }

    pub fn debug_information(&self) -> VideoDebugInformation {
        VideoDebugInformation {
            scroll: self.position_registers.scroll(),
            window: self.position_registers.window(),
            vram: self.vram.clone(),
            control: self.control,
            bg_palette: self.bg_palette,
            obj_palette0: self.obj_palette0,
            obj_palette1: self.obj_palette1,
        }
    }
}

impl Default for Video {
    fn default() -> Video {
        Video {
            control: ControlRegister::new(),
            status: StatusRegister::default(),
            mode: StatusMode::ReadingOAM,
            position_registers: PositionRegisters::default(),
            bg_palette: Palette::from_value(0xFC),
            obj_palette0: Palette::from_value(0xFF),
            obj_palette1: Palette::from_value(0xFF),
            vram: VideoMemory::new(),
            screen: Screen::default(),
            cycles_left: 0,
        }
    }
}

impl Readable for Video {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFE00..=0xFE9F => self.vram.read(address), // oam
            0x9800..=0x9FFF | 0x8000..=0x97FF => self.vram.read(address), // video ram
            0xFF40 => self.control.get(),               // lcdc control
            0xFF41 => self.status.generate(&self),      // lcdc status
            0xFF42 => self.position_registers.scroll().1, // lcdc scroll y
            0xFF43 => self.position_registers.scroll().0, // lcdc scroll x
            0xFF44 => self.position_registers.ly(),     // lcdc LY
            0xFF45 => self.position_registers.lyc(),    // lcdc LYC
            0xFF47 => self.bg_palette.get(),            // background & window palette
            0xFF48 => self.obj_palette0.get(),          // object palette 0
            0xFF49 => self.obj_palette1.get(),          // object palette 1
            0xFF4A => self.position_registers.window().1, // window y position
            0xFF4B => self.position_registers.window().0, // window x position
            _ => unimplemented!(),
        }
    }
}

impl Writable for Video {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFE00..=0xFE9F => self.vram.write(address, value), // oam
            0x9800..=0x9FFF | 0x8000..=0x97FF => self.vram.write(address, value), // video ram
            0xFF40 => self.control.set(value),                  // lcdc control
            0xFF41 => self.status.set(value),                   // lcdc status
            0xFF42 => self.position_registers.set_scroll_y(value, self.mode), // lcdc scroll y
            0xFF43 => self.position_registers.set_scroll_x(value, self.mode), // lcdc scroll x
            0xFF44 => self.position_registers.reset_ly(self.mode), // reset lcdc LY
            0xFF45 => self.position_registers.set_lyc(value, self.mode), // lcdc LYC
            0xFF47 => self.bg_palette.set(value),               // background & window palette
            0xFF48 => self.obj_palette0.set(value),             // object palette 0
            0xFF49 => self.obj_palette1.set(value),             // object palette 1
            0xFF4A => self.position_registers.set_window_y(value, self.mode), // window y position
            0xFF4B => self.position_registers.set_window_x(value, self.mode), // window x position
            _ => unimplemented!(),
        }
    }
}

impl Savestate for Video {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.control.register);
        buffer.push(self.status.register);
        buffer.push(self.mode as u8);
        self.position_registers.dump_savestate(buffer);
        self.bg_palette.dump_savestate(buffer);
        self.obj_palette0.dump_savestate(buffer);
        self.obj_palette1.dump_savestate(buffer);
        self.vram.dump_savestate(buffer);
        write_savestate_u16(buffer, self.cycles_left);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<'a, u8>,
    ) -> Result<(), LoadSavestateError> {
        self.control.register = read_savestate_byte(buffer)?;
        self.status.register = read_savestate_byte(buffer)?;
        self.mode = buffer
            .next()
            .cloned()
            .and_then(StatusMode::from)
            .ok_or(LoadSavestateError::InvalidSavestate)?;
        self.position_registers.load_savestate(buffer)?;
        self.bg_palette.load_savestate(buffer)?;
        self.obj_palette0.load_savestate(buffer)?;
        self.obj_palette1.load_savestate(buffer)?;
        self.vram.load_savestate(buffer)?;
        self.cycles_left = read_savestate_u16(buffer)?;

        Ok(())
    }
}
