use std::error::Error;

use crate::config::Config;
use crate::hardware::Hardware;
use crate::processor::interrupt::Interrupt;
use crate::processor::Processor;
use crate::video::status_register::StatusMode;

pub struct Gameboy {
    processor: Processor,
    hardware: Hardware,
}

impl Gameboy {
    pub fn new(config: Config) -> Result<Gameboy, Box<dyn Error>> {
        Ok(Gameboy {
            processor: Processor::new(config.debugger_config.clone()),
            hardware: Hardware::new(config)?,
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
        let cycles = self.processor.step(&mut self.hardware);
        self.hardware.clock(cycles)
    }

    pub fn hardware(&self) -> &Hardware {
        &self.hardware
    }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor,
}
