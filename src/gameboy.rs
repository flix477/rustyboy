use crate::processor::Processor;
use crate::hardware::Hardware;
use std::error::Error;
use crate::config::Config;
use std::time::Instant;
use crate::util::as_millis;

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
        let mut last_time = Instant::now();
        loop {
            let now = Instant::now();
            let delta = now.duration_since(last_time);
            self.processor.update(&mut self.hardware, as_millis(delta));
            self.hardware.update(as_millis(delta));
            last_time = now;
        }
    }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor
}
