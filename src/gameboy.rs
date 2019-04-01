use crate::config::Config;
use crate::hardware::Hardware;
use crate::processor::Processor;
use std::error::Error;

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
