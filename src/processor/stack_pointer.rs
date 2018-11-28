use bus::Bus;
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

    pub fn peek<H: Bus>(&mut self, bus: &H) -> u8 {
        bus.read(self.value)
    }

    pub fn push<H: Bus>(&mut self, bus: &mut H, value: u16) {
        self.decrement();
        bus.write(self.value, (value >> 8) as u8);
        self.decrement();
        bus.write(self.value, value as u8);
    }

    pub fn pop<H: Bus>(&mut self, bus: &H) -> u16 {
        let low = bus.read(self.value) as u16;
        self.increment();
        let high = bus.read(self.value) as u16;
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