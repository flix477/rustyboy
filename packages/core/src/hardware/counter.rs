use crate::util::savestate::{
    read_savestate_byte, read_savestate_u16, write_savestate_u16,
    LoadSavestateError, Savestate, SavestateStream,
};

#[derive(PartialEq)]
pub enum ClockResult {
    Overflow,
    None,
}

pub struct Counter {
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
