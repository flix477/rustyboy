use crate::processor::Processor;
use crate::hardware::Hardware;
use std::error::Error;
use crate::config::Config;

pub struct Gameboy {
    processor: Processor,
    hardware: Hardware
}

impl Gameboy {
    pub fn new(config: Config) -> Result<Gameboy, Box<dyn Error>> {
        Ok(Gameboy {
            processor: Processor::new(),
            hardware: Hardware::new(config)?
        })
    }

    pub fn update(&mut self, delta: f64) {
        self.processor.update(&mut self.hardware, delta);
//        self.hardware.update(delta);
    }

    pub fn hardware(&self) -> &Hardware { &self.hardware }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor
}
