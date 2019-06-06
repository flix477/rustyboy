use std::error::Error;

use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::hardware::{joypad::Input, Hardware};
use crate::processor::Processor;

pub struct Gameboy {
    processor: Processor,
    hardware: Hardware,
}

impl Gameboy {
    pub fn new(cartridge: Cartridge, config: Config) -> Result<Gameboy, Box<dyn Error>> {
        Ok(Gameboy {
            processor: Processor::new(config.debugger),
            hardware: Hardware::new(cartridge)?,
        })
    }

    pub fn run_to_vblank(&mut self) {
        loop {
            if self.step() {
                break;
            }
        }
    }

    fn step(&mut self) -> bool {
        self.processor.step(&mut self.hardware);
        self.hardware.clock()
    }

    pub fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    pub fn send_input(&mut self, input: Input) {
        self.hardware.send_input(input);
    }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor,
}
