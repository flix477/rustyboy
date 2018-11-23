use memory::Memory;
use processor::register::Register;

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
}

impl Register for ProgramCounter {
    fn get(&self) -> u16 {
        self.value
    }

    fn set(&mut self, value: u16) {
        self.value = value
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn decrement(&mut self) {
        self.value -= 1;
    }
}