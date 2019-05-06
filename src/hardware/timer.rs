use crate::bus::{Readable, Writable};
use crate::processor::interrupt::{Interrupt, InterruptHandler};

const CLOCK_SPEEDS: [usize; 4] = [1024, 16, 64, 256];

pub struct Timer {
    counter_enabled: bool,
    divider: Counter,
    counter: Counter,
    modulo: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            counter_enabled: true,
            divider: Counter::new(CLOCK_SPEEDS[3]),
            counter: Counter::new(CLOCK_SPEEDS[0]),
            modulo: 0,
        }
    }

    pub fn clock(&mut self, interrupt_handler: &mut InterruptHandler) {
        if self.counter_enabled {
            if self.counter.clock() == ClockResult::Overflow {
                self.counter.value = self.modulo;
                interrupt_handler.request_interrupt(Interrupt::Timer)
            }
        }

        self.divider.clock();
    }

    fn control(&self) -> u8 {
        let clock_speed = match self.counter.cycles_per_tick() {
            1024 => 0,
            16 => 1,
            64 => 2,
            256 => 3,
            _ => panic!("Invalid clock speed"),
        };
        clock_speed | ((self.counter_enabled as u8) << 2)
    }

    fn set_control(&mut self, value: u8) {
        self.counter_enabled = (value & 0b100) != 0;
        self.counter
            .set_cycles_per_tick(CLOCK_SPEEDS[value as usize & 0b11]);
    }
}

impl Readable for Timer {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider.value, // divider register
            0xFF05 => self.counter.value, // timer counter
            0xFF06 => self.modulo,        // timer modulo
            0xFF07 => self.control(),     // timer control
            _ => panic!("Invalid address"),
        }
    }
}

impl Writable for Timer {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider.value = 0,     // divider register reset
            0xFF05 => self.counter.value = value, // timer counter
            0xFF06 => self.modulo = value,        // timer modulo
            0xFF07 => self.set_control(value),    // timer control
            _ => panic!("Invalid address"),
        }
    }
}

#[derive(PartialEq)]
enum ClockResult {
    Overflow,
    None,
}

struct Counter {
    cycles_per_tick: usize,
    cycles_left: usize,
    pub value: u8,
}

impl Counter {
    pub fn new(cycles_per_tick: usize) -> Self {
        Self {
            cycles_per_tick,
            cycles_left: cycles_per_tick,
            value: 0,
        }
    }

    pub fn clock(&mut self) -> ClockResult {
        self.cycles_left.saturating_sub(1);
        if self.cycles_left == 0 {
            self.cycles_left = self.cycles_per_tick;
            let (result, overflow) = self.value.overflowing_add(1);
            self.value = result;
            if overflow {
                ClockResult::Overflow
            } else {
                ClockResult::None
            }
        } else {
            ClockResult::None
        }
    }

    pub fn cycles_per_tick(&self) -> usize {
        self.cycles_per_tick
    }

    pub fn set_cycles_per_tick(&mut self, value: usize) {
        // TODO: what happens to self.cycles_left?
        self.cycles_per_tick = value;
    }
}
