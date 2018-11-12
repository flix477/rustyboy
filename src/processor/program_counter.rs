use memory::Memory;

pub struct ProgramCounter {
    value: u16
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            value: 0
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u8 {
        let value = memory.get(self.value);
        self.increment();
        value
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}