use crate::bus::{Bus, Readable, Writable};
use crate::processor::interrupt::Interrupt;

pub struct MockBus {
    pub memory: [u8; 65536],
    pub interrupts_enabled: bool,
}

impl Default for MockBus {
    fn default() -> Self {
        MockBus {
            memory: [0; 65536],
            interrupts_enabled: false,
        }
    }
}

impl Bus for MockBus {
    fn fetch_interrupt(&mut self) -> Option<Interrupt> {
        None
    }
    fn request_interrupt(&mut self, _: Interrupt) {}
    fn toggle_interrupts(&mut self, value: bool) {
        self.interrupts_enabled = value;
    }
    fn dma_transfer(&mut self, _: u16, _: u16, _: u16) {}
}

impl Readable for MockBus {
    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl Writable for MockBus {
    fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
