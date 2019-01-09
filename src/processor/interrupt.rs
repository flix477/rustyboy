use crate::util::bitflags::Bitflags;
use crate::bus::{Readable, Writable};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Interrupt {
    VBlank = 1,
    LCDCStat = 2,
    Timer = 4,
    Serial = 8,
    Keypad = 16
}

impl Interrupt {
    pub fn address(&self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::LCDCStat => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Keypad => 0x0060
        }
    }
}

impl Into<u8> for Interrupt {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Interrupt {
    fn from(value: u8) -> Interrupt {
        match value {
            1 => Interrupt::VBlank,
            2 => Interrupt::LCDCStat,
            4 => Interrupt::Timer,
            8 => Interrupt::Serial,
            16 => Interrupt::Keypad,
            _ => panic!("Invalid value.")
        }
    }
}

pub struct InterruptRegister {
    register: u8
}

impl InterruptRegister {
    pub fn new() -> InterruptRegister {
        return InterruptRegister {
            register: 0
        };
    }

    pub fn from_value(value: u8) -> InterruptRegister {
        return InterruptRegister {
            register: value
        };
    }
}

impl Bitflags<Interrupt> for InterruptRegister {
    fn register(&self) -> u8 {
        self.register
    }
    fn set_register(&mut self, value: u8) {
        self.register = value;
    }
}

pub struct InterruptHandler {
    interrupt_request: InterruptRegister,
    interrupt_enable: InterruptRegister,
    interrupt_master_enable: bool
}

impl InterruptHandler {
    pub fn new() -> InterruptHandler {
        InterruptHandler {
            interrupt_request: InterruptRegister::new(),
            interrupt_enable: InterruptRegister::from_value(0xFF),
            interrupt_master_enable: false
        }
    }

    pub fn fetch_interrupt(&mut self) -> Option<Interrupt> {
        let mask = if self.interrupt_master_enable { 0xFF } else { 0 };
        let value = mask & self.interrupt_enable.register() & self.interrupt_request.register();

        for x in 0..=4 {
            let interrupt = Interrupt::from(2u8.pow(x as u32));
            if (value & interrupt as u8) != 0 {
                self.interrupt_master_enable = false;
                self.interrupt_request.set_flag(interrupt, false);
                return Some(interrupt)
            }
        }

        None
    }

    pub fn toggle_interrupts(&mut self, value: bool) {
        self.interrupt_master_enable = value;
    }

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_request.set_flag(interrupt, true);
    }
}

impl Readable for InterruptHandler {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFFFF => self.interrupt_master_enable as u8,
            0xFF0F => self.interrupt_request.register,
            _ => 0
        }
    }
}

impl Writable for InterruptHandler {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFFFF => self.interrupt_master_enable = value == 1,
            0xFF0F => self.interrupt_request.register = value,
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_interrupt_ime_false() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler.interrupt_master_enable = false;
        interrupt_handler.interrupt_request.set_flag(Interrupt::Keypad, true);
        assert!(interrupt_handler.fetch_interrupt().is_none())
    }

    #[test]
    fn fetch_interrupt_ie_false() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler.interrupt_enable.set_flag(Interrupt::Serial, false);
        interrupt_handler.interrupt_request.set_flag(Interrupt::Serial, true);
        assert!(interrupt_handler.fetch_interrupt().is_none())
    }

    #[test]
    fn fetch_interrupt_none() {
        let mut interrupt_handler = InterruptHandler::new();
        assert!(interrupt_handler.fetch_interrupt().is_none())
    }

    #[test]
    fn fetch_interrupt_one() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler.interrupt_request.set_flag(Interrupt::LCDCStat, true);
        let interrupt = interrupt_handler.fetch_interrupt().unwrap();
        assert_eq!(interrupt, Interrupt::LCDCStat);
    }

    #[test]
    fn fetch_interrupt_multiple() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler.interrupt_request.set_register(0xFF);
        assert_eq!(interrupt_handler.fetch_interrupt().unwrap(), Interrupt::VBlank);
        interrupt_handler.interrupt_master_enable = true;
        assert_eq!(interrupt_handler.fetch_interrupt().unwrap(), Interrupt::LCDCStat);
        interrupt_handler.interrupt_master_enable = true;
        assert_eq!(interrupt_handler.fetch_interrupt().unwrap(), Interrupt::Timer);
        interrupt_handler.interrupt_master_enable = true;
        assert_eq!(interrupt_handler.fetch_interrupt().unwrap(), Interrupt::Serial);
        interrupt_handler.interrupt_master_enable = true;
        assert_eq!(interrupt_handler.fetch_interrupt().unwrap(), Interrupt::Keypad);
        interrupt_handler.interrupt_master_enable = true;
        assert!(interrupt_handler.fetch_interrupt().is_none());
    }
}