use crate::bus::{Readable, Writable, Bus};
use crate::processor::interrupt::{Interrupt, InterruptHandler};

// TODO: move this to config singleton thingy
const CPU_CLOCK_SPEED: f64 = 4194304.0; // Hz
const CLOCK_SPEEDS: [usize; 4] = [1024, 16, 64, 256];

pub struct Timer {
    counter_enabled: bool,
    divider: u8,
    counter: u8,
    modulo: u8,
    clock_speed: usize, // is actually cpu clock speed divided by this value
    leftover_time: f64
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            counter_enabled: true,
            divider: 0,
            counter: 0,
            modulo: 0,
            clock_speed: CLOCK_SPEEDS[0],
            leftover_time: 0.0
        }
    }

    fn time_per_tick(&self) -> f64 {
        1.0 / (CPU_CLOCK_SPEED / self.clock_speed as f64)
    }

    pub fn update(&mut self, interrupt_handler: &mut InterruptHandler, delta: f64) {
        self.leftover_time += delta;
        while self.leftover_time >= self.time_per_tick() {
            self.leftover_time -= self.time_per_tick();
            self.tick(interrupt_handler);
        }
    }

    fn tick(&mut self, interrupt_handler: &mut InterruptHandler) {
        if self.counter_enabled {
            if self.counter == 0xFF {
                self.counter = self.modulo;
                interrupt_handler.request_interrupt(Interrupt::Timer)
            } else {
                self.counter += 1;
            }
        }
        self.divider.wrapping_add(1);
    }

    fn control(&self) -> u8 {
        let clock_speed = match self.clock_speed {
            1024 => 0,
            16 => 1,
            64 => 2,
            256 => 3,
            _ => panic!("Invalid clock speed")
        };
        clock_speed | ((self.counter_enabled as u8) << 2)
    }

    fn set_control(&mut self, value: u8) {
        self.counter_enabled = (value & 0b100) == 1;
        self.clock_speed = CLOCK_SPEEDS[value as usize & 0b11];
    }
}

impl Readable for Timer {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider, // divider register
            0xFF05 => self.counter, // timer counter
            0xFF06 => self.modulo, // timer modulo
            0xFF07 => self.control(), // timer control
            _ => panic!("Invalid address")
        }
    }
}

impl Writable for Timer {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider = 0, // divider register reset
            0xFF05 => self.counter = value, // timer counter
            0xFF06 => self.modulo = value, // timer modulo
            0xFF07 => self.set_control(value), // timer control
            _ => panic!("Invalid address")
        }
    }
}