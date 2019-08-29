use crate::bus::{Readable, Writable};
use crate::processor::interrupt::{Interrupt, InterruptHandler};
use crate::util::savestate::{
    read_savestate_bool, read_savestate_byte, read_savestate_u16, write_savestate_u16,
    LoadSavestateError, Savestate, SavestateStream,
};

const CLOCK_SPEEDS: [u16; 4] = [1024, 16, 64, 256];

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
        if self.counter_enabled && self.counter.clock() == ClockResult::Overflow {
            self.counter.value = self.modulo;
            interrupt_handler.request_interrupt(Interrupt::Timer)
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

impl Savestate for Timer {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.counter_enabled as u8);
        self.divider.dump_savestate(buffer);
        self.counter.dump_savestate(buffer);
        buffer.push(self.modulo);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream<'a>,
    ) -> Result<(), LoadSavestateError> {
        self.counter_enabled = read_savestate_bool(buffer)?;
        self.divider.load_savestate(buffer)?;
        self.counter.load_savestate(buffer)?;
        self.modulo = read_savestate_byte(buffer)?;
        Ok(())
    }
}

#[derive(PartialEq)]
enum ClockResult {
    Overflow,
    None,
}

struct Counter {
    cycles_per_tick: u16,
    cycles_left: u16,
    pub value: u8,
}

impl Counter {
    pub fn new(cycles_per_tick: u16) -> Self {
        Self {
            cycles_per_tick,
            cycles_left: cycles_per_tick,
            value: 0,
        }
    }

    pub fn clock(&mut self) -> ClockResult {
        self.cycles_left = self.cycles_left.saturating_sub(1);
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

    pub fn cycles_per_tick(&self) -> u16 {
        self.cycles_per_tick
    }

    pub fn set_cycles_per_tick(&mut self, value: u16) {
        // TODO: what happens to self.cycles_left?
        self.cycles_per_tick = value;
    }
}

impl Savestate for Counter {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        write_savestate_u16(buffer, self.cycles_per_tick);
        write_savestate_u16(buffer, self.cycles_left);
        buffer.push(self.value);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream<'a>,
    ) -> Result<(), LoadSavestateError> {
        self.cycles_per_tick = read_savestate_u16(buffer)?;
        self.cycles_left = read_savestate_u16(buffer)?;
        self.value = read_savestate_byte(buffer)?;
        Ok(())
    }
}
