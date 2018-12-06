use processor::Processor;
use hardware::Hardware;
use std::error::Error;
use config::Config;

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

    pub fn start(&mut self) {
        loop {
            self.processor.step(&mut self.hardware);
        }
    }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor
}
