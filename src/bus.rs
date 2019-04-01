use crate::processor::interrupt::Interrupt;

pub trait Readable {
    fn read(&self, address: u16) -> u8;
}

pub trait Writable {
    fn write(&mut self, address: u16, value: u8);
}

pub trait Bus: Readable + Writable {
    fn fetch_interrupt(&mut self) -> Option<Interrupt>;
    fn request_interrupt(&mut self, interrupt: Interrupt);
    fn toggle_interrupts(&mut self, value: bool);
    fn dma_transfer(&mut self, from: u16, to: u16, size: u16);
}
