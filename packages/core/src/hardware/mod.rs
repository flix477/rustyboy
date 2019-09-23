use crate::bus::{Bus, Readable, Writable};
use crate::cartridge::Cartridge;
use crate::processor::interrupt::{Interrupt, InterruptHandler};
use crate::sound::Sound;
use crate::video::Video;

use self::joypad::{Input, Joypad};
use self::timer::Timer;
use crate::util::savestate::{read_savestate_byte, LoadSavestateError, Savestate, SavestateStream};
use crate::video::status_register::StatusMode;

pub mod joypad;
mod timer;

pub struct Hardware {
    pub cartridge: Cartridge,
    interrupt_handler: InterruptHandler,
    joypad: Joypad,
    timer: Timer,
    pub video: Video,
    internal_ram: [u8; 8192],
    high_ram: [u8; 127],
    sound: Sound
}

impl Hardware {
    pub fn new(cartridge: Cartridge) -> Hardware {
        Hardware {
            cartridge,
            interrupt_handler: InterruptHandler::new(),
            joypad: Joypad::new(),
            timer: Timer::new(),
            video: Video::default(),
            internal_ram: [0; 8192],
            high_ram: [0; 127],
            sound: Sound::default()
        }
    }

    // TODO: can we find a way to just make a new Hardware
    // instead so we don't duplicate the constructor?
    // I'm also a bit worried about having to clone cartridge around
    pub fn reset(&mut self) {
        self.cartridge.reset();
        self.high_ram = [0; 127];
        self.internal_ram = [0; 8192];
        self.video = Video::default();
        self.timer = Timer::new();
        self.joypad = Joypad::new();
        self.interrupt_handler = InterruptHandler::new();
    }

    pub fn clock(&mut self) -> Option<StatusMode> {
        self.timer.clock(&mut self.interrupt_handler);
        self.video.clock(&mut self.interrupt_handler)
    }

    pub fn interrupt_handler(&self) -> &InterruptHandler {
        &self.interrupt_handler
    }

    fn audio_unimplemented(&self) {}

    pub fn send_input(&mut self, input: Input) {
        self.joypad.send_input(input);
    }
}

impl Readable for Hardware {
    fn read(&self, address: u16) -> u8 {
        match address {
            0..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(address), // cartridge

            0xFF46 => 0xFF, // dma transfer

            0xFF40..=0xFF45 | 0xFF47..=0xFF4B | 0x8000..=0x9FFF | 0xFE00..=0xFE9F => {
                self.video.read(address)
            } // lcdc|video ram,

            0xC000..=0xCFFF => {
                let address = address - 0xC000;
                self.internal_ram[address as usize]
            } // 4kb internal ram
            0xD000..=0xDFFF => {
                // TODO: cgb internal ram bank switching
                let address = address - 0xC000;
                self.internal_ram[address as usize]
            } // 4kb internal ram bank
            0xE000..=0xFDFF => {
                let address = address - 0xE000;
                self.internal_ram[address as usize]
            } // echo ^^

            0xFF4C..=0xFF7F | 0xFEA0..=0xFEFF => 0xFF, // empty but unusable for i/o
            0xFF00 => self.joypad.read(address), // joypad info
            0xFF01 => {
                // TODO: serial transfer data
                0
            } // serial transfer data
            0xFF02 => {
                // TODO: sio control
                0
            } // sio control
            0xFF04..=0xFF07 => self.timer.read(address), // timer
            0xFF0F | 0xFFFF => self.interrupt_handler.read(address), // interrupt
            0xFF10..=0xFF3F => self.sound.read(address), // apu
            0xFF80..=0xFFFE => {
                let address = address - 0xFF80;
                self.high_ram[address as usize]
            } // high ram

            _ => {
                // println!("Unrecognised read at 0x{:X}", address);
                0xFF
                // unimplemented!()
            } // empty
        }
    }
}

impl Writable for Hardware {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write(address, value), // cartridge

            0xFF46 => self.dma_transfer(u16::from(value) * 0x100, 0xFE00, 0x9F), // dma transfer

            0xFF40..=0xFF45 | 0xFF47..=0xFF4B | 0xFE00..=0xFE9F | 0x8000..=0x9FFF => {
                self.video.write(address, value)
            } // lcdc|sprite attrib|video ram

            0xC000..=0xCFFF => {
                let address = address - 0xC000;
                self.internal_ram[address as usize] = value;
            } // 4kb internal ram
            0xD000..=0xDFFF => {
                // TODO: cgb internal ram bank switching
                let address = address - 0xC000;
                self.internal_ram[address as usize] = value;
            } // 4kb internal ram bank
            0xE000..=0xFDFF => {
                let address = address - 0xE000;
                self.internal_ram[address as usize] = value;
            } // echo ^^

            0xFF4C..=0xFF7F | 0xFEA0..=0xFEFF => {} // empty but unusable for i/o

            0xFF00 => self.joypad.write(address, value), // joypad

            0xFF01 => {
                // TODO serial transfer data
            } // serial transfer data
            0xFF02 => {
                // TODO sio control
            } // sio control

            0xFF04..=0xFF07 => self.timer.write(address, value), // timer
            0xFF0F | 0xFFFF => self.interrupt_handler.write(address, value), // interrupt enable (IE)
            0xFF10..=0xFF3F => self.sound.write(address, value), // apu

            0xFF80..=0xFFFE => {
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

impl Savestate for Hardware {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        self.cartridge.dump_savestate(buffer);
        self.interrupt_handler.dump_savestate(buffer);
        self.timer.dump_savestate(buffer);
        self.video.dump_savestate(buffer);
        buffer.append(&mut self.internal_ram.to_vec());
        buffer.append(&mut self.high_ram.to_vec());
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream<'a>,
    ) -> Result<(), LoadSavestateError> {
        self.cartridge.load_savestate(buffer)?;
        self.interrupt_handler.load_savestate(buffer)?;
        self.timer.load_savestate(buffer)?;
        self.video.load_savestate(buffer)?;

        for i in 0..self.internal_ram.len() {
            self.internal_ram[i] = read_savestate_byte(buffer)?;
        }

        for i in 0..self.high_ram.len() {
            self.high_ram[i] = read_savestate_byte(buffer)?;
        }

        Ok(())
    }
}
