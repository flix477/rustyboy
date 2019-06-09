// TODO: implement writing to clock

pub struct RealTimeClock {
    active_register: RTCRegister,
    latched_state: ClockState,
    prelatch_triggered: bool,
}

impl RealTimeClock {
    pub fn new() -> RealTimeClock {
        RealTimeClock {
            active_register: RTCRegister::Seconds, // TODO: is it tho
            latched_state: ClockState::now(),
            prelatch_triggered: false,
        }
    }

    pub fn active_value(&self) -> u8 {
        match self.active_register() {
            RTCRegister::Seconds => self.latched_state.seconds,
            RTCRegister::Minutes => self.latched_state.minutes,
            RTCRegister::Hours => self.latched_state.hours,
            RTCRegister::DayLow => self.latched_state.day_counter as u8,
            RTCRegister::DayHigh => (self.latched_state.day_counter >> 8) as u8,
        }
    }

    pub fn active_register(&self) -> &RTCRegister {
        &self.active_register
    }

    pub fn set_active_register(&mut self, register: RTCRegister) {
        self.active_register = register;
    }

    pub fn latch(&mut self) {
        if !self.prelatch_triggered {
            self.prelatch_triggered = true;
            return;
        }

        self.latched_state = ClockState::now()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RTCRegister {
    Seconds,
    Minutes,
    Hours,
    DayLow,
    DayHigh,
}

impl RTCRegister {
    pub fn from_value(value: u8) -> Option<RTCRegister> {
        match value {
            8 => Some(RTCRegister::Seconds),
            9 => Some(RTCRegister::Minutes),
            0xA => Some(RTCRegister::Hours),
            0xB => Some(RTCRegister::DayLow),
            0xC => Some(RTCRegister::DayHigh),
            _ => None,
        }
    }
}

pub struct ClockState {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day_counter: u16,
}

impl ClockState {
    pub fn now() -> ClockState {
        ClockState {
            seconds: 0,
            minutes: 0,
            hours: 0,
            day_counter: 0,
        }
    }
}
