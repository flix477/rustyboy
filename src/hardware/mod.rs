use std::error::Error;

use crate::bus::{Bus, Readable, Writable};
use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::processor::interrupt::{Interrupt, InterruptHandler};
use crate::video::Video;

use self::joypad::Joypad;
use self::timer::Timer;

mod joypad;
mod timer;

pub struct Hardware {
    cartridge: Cartridge,
    interrupt_handler: InterruptHandler,
    joypad: Joypad,
    timer: Timer,
    video: Video,
    internal_ram: [u8; 8192],
    high_ram: [u8; 127],
}

impl Hardware {
    pub fn new(config: Config) -> Result<Hardware, Box<dyn Error>> {
        Ok(Hardware {
            cartridge: config.cartridge,
            interrupt_handler: InterruptHandler::new(),
            joypad: Joypad::new(),
            timer: Timer::new(),
            video: Video::new(),
            internal_ram: [0; 8192],
            high_ram: [0; 127],
        })
    }

    pub fn clock(&mut self) -> bool {
        self.timer.clock(&mut self.interrupt_handler);
        self.video.clock(&mut self.interrupt_handler)
    }

    pub fn video(&self) -> &Video {
        &self.video
    }
    pub fn interrupt_handler(&self) -> &InterruptHandler {
        &self.interrupt_handler
    }

    fn audio_unimplemented(&self) {}
}

impl Readable for Hardware {
    fn read(&self, address: u16) -> u8 {
        match address {
            0...0x7FFF | 0xA000...0xBFFF => self.cartridge.read(address), // cartridge

            0xFF46 => {
                //                0
                unimplemented!()
            } // dma transfer
            0xFF40...0xFF4B | 0x8000...0x9FFF | 0xFE00...0xFE9F => self.video.read(address), // lcdc|video ram,

            0xC000...0xCFFF => {
                let address = address - 0xC000;
                self.internal_ram[address as usize]
            } // 4kb internal ram
            0xD000...0xDFFF => {
                // TODO: cgb internal ram bank switching
                let address = address - 0xC000;
                self.internal_ram[address as usize]
            } // 4kb internal ram bank
            0xE000...0xFDFF => {
                let address = address - 0xE000;
                self.internal_ram[address as usize]
            } // echo ^^

            0xFF4C...0xFF7F | 0xFEA0...0xFEFF => 0, // empty but unusable for i/o

            0xFF00 => self.joypad.read(address), // joypad info

            0xFF01 => {
                // TODO: serial transfer data
                0
            } // serial transfer data
            0xFF02 => {
                // TODO: sio control
                0
            } // sio control

            0xFF04...0xFF07 => self.timer.read(address), // timer

            0xFF0F | 0xFFFF => self.interrupt_handler.read(address), // interrupt

            0xFF10 => 0,          // sound mode 1 register, sweep
            0xFF11 => 0,          // sound mode 1 register, sound length
            0xFF12 => 0,          // sound mode 1 register, envelope
            0xFF13 => 0,          // sound mode 1 register, frequency low
            0xFF14 => 0,          // sound mode 1 register, frequency high
            0xFF16 => 0,          // sound mode 2 register, sound length
            0xFF17 => 0,          // sound mode 2 register, envelope
            0xFF18 => 0,          // sound mode 2 register, frequency low
            0xFF19 => 0,          // sound mode 2 register, frequency high
            0xFF1A => 0,          // sound mode 3 register, on/off
            0xFF1B => 0,          // sound mode 3 register, sound length
            0xFF1C => 0,          // sound mode 3 register, output level
            0xFF1D => 0,          // sound mode 3 register, frequency low
            0xFF1E => 0,          // sound mode 3 register, frequency high
            0xFF20 => 0,          // sound mode 4 register, sound length
            0xFF21 => 0,          // sound mode 4 register, envelope
            0xFF22 => 0,          // sound mode 4 register, polynomial counter
            0xFF23 => 0,          // sound mode 4 register, counter/consecutive
            0xFF24 => 0,          // channel control - on/off - volume
            0xFF25 => 0,          // sound output terminal selection
            0xFF26 => 0,          // sound on/off
            0xFF30...0xFF3F => 0, // waveform ram

            0xFF80...0xFFFE => {
                let address = address - 0xFF80;
                self.high_ram[address as usize]
            } // high ram

            _ => {
                // println!("Unrecognised read at 0x{:X}", address);
                0
                // unimplemented!()
            } // empty
        }
    }
}

impl Writable for Hardware {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0...0x7FFF | 0xA000...0xBFFF => self.cartridge.write(address, value), // cartridge

            0xFF46 => self.dma_transfer(value as u16 * 0x100, 0xFE00, 0x9F), // dma transfer
            0xFF40...0xFF4B | 0xFE00...0xFE9F | 0x8000...0x9FFF => self.video.write(address, value), // lcdc|sprite attrib|video ram

            0xC000...0xCFFF => {
                let address = address - 0xC000;
                self.internal_ram[address as usize] = value;
            } // 4kb internal ram
            0xD000...0xDFFF => {
                // TODO: cgb internal ram bank switching
                let address = address - 0xC000;
                self.internal_ram[address as usize] = value;
            } // 4kb internal ram bank
            0xE000...0xFDFF => {
                let address = address - 0xE000;
                self.internal_ram[address as usize] = value;
            } // echo ^^

            0xFF4C...0xFF7F | 0xFEA0...0xFEFF => {} // empty but unusable for i/o

            0xFF00 => self.joypad.write(address, value), // joypad

            0xFF01 => {
                // TODO serial transfer data
            } // serial transfer data
            0xFF02 => {
                // TODO sio control
            } // sio control

            0xFF04...0xFF07 => self.timer.write(address, value), // timer

            0xFF0F | 0xFFFF => self.interrupt_handler.write(address, value), // interrupt enable (IE)

            0xFF10 => self.audio_unimplemented(), // sound mode 1 register, sweep
            0xFF11 => self.audio_unimplemented(), // sound mode 1 register, sound length
            0xFF12 => self.audio_unimplemented(), // sound mode 1 register, envelope
            0xFF13 => self.audio_unimplemented(), // sound mode 1 register, frequency low
            0xFF14 => self.audio_unimplemented(), // sound mode 1 register, frequency high
            0xFF16 => self.audio_unimplemented(), // sound mode 2 register, sound length
            0xFF17 => self.audio_unimplemented(), // sound mode 2 register, envelope
            0xFF18 => self.audio_unimplemented(), // sound mode 2 register, frequency low
            0xFF19 => self.audio_unimplemented(), // sound mode 2 register, frequency high
            0xFF1A => self.audio_unimplemented(), // sound mode 3 register, on/off
            0xFF1B => self.audio_unimplemented(), // sound mode 3 register, sound length
            0xFF1C => self.audio_unimplemented(), // sound mode 3 register, output level
            0xFF1D => self.audio_unimplemented(), // sound mode 3 register, frequency low
            0xFF1E => self.audio_unimplemented(), // sound mode 3 register, frequency high
            0xFF20 => self.audio_unimplemented(), // sound mode 4 register, sound length
            0xFF21 => self.audio_unimplemented(), // sound mode 4 register, envelope
            0xFF22 => self.audio_unimplemented(), // sound mode 4 register, polynomial counter
            0xFF23 => self.audio_unimplemented(), // sound mode 4 register, counter/consecutive
            0xFF24 => self.audio_unimplemented(), // channel control - on/off - volume
            0xFF25 => self.audio_unimplemented(), // sound output terminal selection
            0xFF26 => self.audio_unimplemented(), // sound on/off
            0xFF30...0xFF3F => self.audio_unimplemented(), // waveform ram

            0xFF80...0xFFFE => {
                let address = address - 0xFF80;
                self.high_ram[address as usize] = value;
            } // high ram

            _ => {
                // println!("Unrecognised write at 0x{:X}: {}", address, value);
                // unimplemented!()
            } // empty
        }
    }
}

impl Bus for Hardware {
    fn fetch_interrupt(&self) -> Option<Interrupt> {
        self.interrupt_handler.fetch_interrupt()
    }

    fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_handler.request_interrupt(interrupt);
    }

    fn service_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_handler.service_interrupt(interrupt);
    }

    fn toggle_interrupts(&mut self, value: bool) {
        self.interrupt_handler.toggle_interrupts(value);
    }

    fn dma_transfer(&mut self, from: u16, to: u16, size: u16) {
        for i in 0..=size {
            let value = self.read(from + i);
            self.write(to + i, value);
        }
    }

    fn master_interrupt_enable(&self) -> bool {
        self.interrupt_handler.master_interrupt_enable()
    }
}
