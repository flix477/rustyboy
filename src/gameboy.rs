use processor::Processor;
use bus::Hardware;
use std::error::Error;

pub struct Gameboy {
    processor: Processor,
    hardware: Hardware
}

impl Gameboy {
    pub fn new() -> Result<Gameboy, Box<dyn Error>> {
        Ok(Gameboy {
            processor: Processor::new(),
            hardware: Hardware::new()?
        })
    }

    pub fn start(&mut self) {
        self.processor.step(&mut self.hardware);
    }

//    pub fn reset(&mut self) {
//        self.map = [0; MEMORY_MAP_LENGTH];
//    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn is_empty() {
//        let memory = Memory::new();
//        assert!(memory.map.iter().all(|x| { *x == 0 }));
//    }
//
//    #[test]
//    fn resets() {
//        let mut memory = Memory::new();
//        memory.map[1] = 3;
//        memory.reset();
//        assert!(memory.map.iter().all(|x| { *x == 0 }));
//    }
//}