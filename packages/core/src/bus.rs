use crate::processor::interrupt::Interrupt;

pub trait Readable {
    fn read(&self, address: u16) -> u8;

    fn read_all(&self) -> Vec<u8> {
        (0..0x10000)
            .map(|address| self.read(address as u16))
            .collect()
    }
}

pub trait Writable {
    fn write(&mut self, address: u16, value: u8);
}

pub trait Bus: Readable + Writable {
    fn fetch_interrupt(&self) -> Option<Interrupt>;
    fn request_interrupt(&mut self, interrupt: Interrupt);
    fn service_interrupt(&mut self, interrupt: Interrupt);
    fn toggle_interrupts(&mut self, value: bool);
    fn dma_transfer(&mut self, from: u16, to: u16, size: u16);
    fn master_interrupt_enable(&self) -> bool;
}
