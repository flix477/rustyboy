use memory::Memory;
use processor::register::Register;

pub struct StackPointer {
    value: u16
}

impl StackPointer {
    pub fn new() -> StackPointer {
        StackPointer {
            value: 0
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u8 {
        let value = memory.get(self.value);
        self.increment();
        value
    }

    pub fn push(&mut self, memory: &mut Memory, value: u8) {
        self.decrement();
        memory.set(self.value, value);
    }

    pub fn pop(&mut self, memory: &Memory) -> u8 {
        let value = memory.get(self.value);
        self.increment();
        value
    }
}

impl Register for StackPointer {
    fn get(&self) -> u16 {
        self.value
    }

    fn set(&mut self, value: u16) {
        self.value = value;
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn decrement(&mut self) {
        self.value -= 1;
    }
}