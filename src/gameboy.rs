use crate::config::Config;
use crate::hardware::Hardware;
use crate::processor::Processor;
use std::error::Error;
use crate::debugger::Debugger;

pub struct Gameboy<'a> {
    processor: Processor,
    hardware: Hardware,
    debugger: Debugger<'a>
}

impl<'a> Gameboy<'a> {
    pub fn new(config: Config) -> Result<Gameboy<'a>, Box<dyn Error>> {
        Ok(Gameboy {
            processor: Processor::new(),
            hardware: Hardware::new(config)?,
            debugger: Debugger::new()
        })
    }

    pub fn update(&mut self, delta: f64) {
        self.processor.update(&mut self.hardware, delta);
        self.hardware.update(delta);
    }

    pub fn hardware(&self) -> &Hardware {
        &self.hardware
    }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor,
}
