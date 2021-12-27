use crate::bus::{Readable, Writable};
use crate::hardware::counter::{Counter, ClockResult};
use crate::processor::interrupt::{InterruptHandler, Interrupt};
use crate::util::bits;

pub struct Serial {
    data: u8,
    control: u8,
    counter: Counter
}

enum SerialClockSpeed {
    Normal,
    Fast
}

enum SerialClock {
    Internal,
    External
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            data: 0,
            control: 1,
            counter: Counter::new(512)
        }
    }
}

impl Serial {
    fn is_transfer_in_progress_or_requested(&self) -> bool {
        return bits::get_bit(self.control, 7);
    }

    fn clock_speed(&self) -> SerialClockSpeed {
        if bits::get_bit(self.control, 1) {
            // TODO only for cgb mode
            SerialClockSpeed::Fast
        } else {
            SerialClockSpeed::Normal
        }
    }

    fn shift_clock(&self) -> SerialClock {
        if bits::get_bit(self.control, 0) {
            SerialClock::Internal
        } else {
            SerialClock::External
        }
    }
}

impl Serial {
    pub fn clock(&mut self, interrupt_handler: &mut InterruptHandler, serial_input: Option<u8>) {
        if self.counter.clock() == ClockResult::Overflow {
            self.data = serial_input.unwrap_or(0xFF);
            interrupt_handler.request_interrupt(Interrupt::Serial)
        }
    }
}

impl Readable for Serial {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data, // serial transfer data
            0xFF02 => self.control, // sio control
            _ => unimplemented!()
        }
    }
}

impl Writable for Serial {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => { self.data = value },
            0xFF02 => { self.control = value },
            _ => unimplemented!()
        }
    }
}
