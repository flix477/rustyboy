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

    pub fn peek(&mut self, memory: &Memory) -> u8 {
        memory.get(self.value)
    }

    pub fn push(&mut self, memory: &mut Memory, value: u16) {
        self.decrement();
        memory.set(self.value, (value >> 8) as u8);
        self.decrement();
        memory.set(self.value, value as u8);
    }

    pub fn pop(&mut self, memory: &Memory) -> u16 {
        let low = memory.get(self.value) as u16;
        self.increment();
        let high = memory.get(self.value) as u16;
        self.increment();
        low | (high << 8)
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