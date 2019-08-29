use crate::bus::{Readable, Writable};
use crate::util::bitflags::Bitflags;
use crate::util::savestate::{
    read_savestate_bool, read_savestate_byte, LoadSavestateError, Savestate, SavestateStream,
};

/// Represents a hardware interrupt.
///
/// Interrupts are events that occur during the execution of the GameBoy
/// and are requested by one its components, like the keypad when a button is pressed
/// or the PPU when it gets to a VBlank.
///
/// When an interrupt request is made by a component, the processor can service it
/// on its next execution step by jumping to the address that corresponds to the interrupt,
/// where code to handle it resides.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Interrupt {
    VBlank = 1,
    LCDCStat = 2,
    Timer = 4,
    Serial = 8,
    Keypad = 16,
}

impl Interrupt {
    /// Returns the address that corresponds to the interrupt.
    /// The processor will jump to this location when servicing the interrupt.
    pub fn address(self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::LCDCStat => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Keypad => 0x0060,
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
            _ => panic!("Invalid value."),
        }
    }
}

#[derive(Default)]
pub struct InterruptRegister {
    register: u8,
}

impl InterruptRegister {
    pub fn new() -> InterruptRegister {
        InterruptRegister::default()
    }

    pub fn from_value(value: u8) -> InterruptRegister {
        InterruptRegister { register: value }
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

impl Savestate for InterruptRegister {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.register);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream<'a>,
    ) -> Result<(), LoadSavestateError> {
        self.register = read_savestate_byte(buffer)?;
        Ok(())
    }
}

/// This struct contains the interrupt request register (IF), the interrupt enable register (IE)
/// and the interrupt master enable register (IME).
///
/// The IF register contains information about interrupt requests.
/// Every time an interrupt request comes through, the corresponding bit is set in this register.
///
/// The IE register is used to enable or disable the servicing of specific interrupts.
///
/// The IME register is used to completely disable or enable all interrupts at once.
pub struct InterruptHandler {
    interrupt_request: InterruptRegister,
    interrupt_enable: InterruptRegister,
    interrupt_master_enable: bool,
}

impl InterruptHandler {
    pub fn new() -> InterruptHandler {
        InterruptHandler::default()
    }

    /// Returns the next interrupt to service if there is one.
    ///
    /// If multiple interrupts were requested, they are serviced in the order
    /// that they appear in the `Interrupt` enum.
    pub fn fetch_interrupt(&self) -> Option<Interrupt> {
        let value = self.interrupt_enable.register() & self.interrupt_request.register();

        for x in 0..=4 {
            let interrupt = Interrupt::from(2u8.pow(x as u32));
            if (value & interrupt as u8) != 0 {
                return Some(interrupt);
            }
        }

        None
    }

    /// Toggles on or off the interrupt master enable register
    pub fn toggle_interrupts(&mut self, value: bool) {
        self.interrupt_master_enable = value;
    }

    /// Requests an interrupt
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_request.set_flag(interrupt, true);
    }

    /// Toggles IME off and unsets this interrupt in the IF register,
    /// used before the CPU actually services the interrupt
    pub fn service_interrupt(&mut self, interrupt: Interrupt) {
        self.toggle_interrupts(false);
        self.interrupt_request.set_flag(interrupt, false);
    }

    pub fn master_interrupt_enable(&self) -> bool {
        self.interrupt_master_enable
    }
}

impl Default for InterruptHandler {
    fn default() -> InterruptHandler {
        InterruptHandler {
            interrupt_request: InterruptRegister::new(),
            interrupt_enable: InterruptRegister::from_value(0xFF),
            interrupt_master_enable: false,
        }
    }
}

impl Readable for InterruptHandler {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFFFF => self.interrupt_enable.register as u8,
            0xFF0F => self.interrupt_request.register,
            _ => 0,
        }
    }
}

impl Writable for InterruptHandler {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFFFF => self.interrupt_enable.register = value,
            0xFF0F => self.interrupt_request.register = value,
            _ => {}
        }
    }
}

impl Savestate for InterruptHandler {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        self.interrupt_request.dump_savestate(buffer);
        self.interrupt_enable.dump_savestate(buffer);
        buffer.push(self.interrupt_master_enable as u8);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream<'a>,
    ) -> Result<(), LoadSavestateError> {
        self.interrupt_request.load_savestate(buffer)?;
        self.interrupt_request.load_savestate(buffer)?;
        self.interrupt_master_enable = read_savestate_bool(buffer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_interrupt_ie_false() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler
            .interrupt_enable
            .set_flag(Interrupt::Serial, false);
        interrupt_handler
            .interrupt_request
            .set_flag(Interrupt::Serial, true);
        assert!(interrupt_handler.fetch_interrupt().is_none())
    }

    #[test]
    fn fetch_interrupt_none() {
        let interrupt_handler = InterruptHandler::new();
        assert!(interrupt_handler.fetch_interrupt().is_none())
    }

    #[test]
    fn fetch_interrupt_one() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler.toggle_interrupts(true);
        interrupt_handler
            .interrupt_request
            .set_flag(Interrupt::LCDCStat, true);
        let interrupt = interrupt_handler.fetch_interrupt().unwrap();
        assert_eq!(interrupt, Interrupt::LCDCStat);
    }

    #[test]
    fn fetch_interrupt_multiple() {
        let mut interrupt_handler = InterruptHandler::new();
        interrupt_handler.toggle_interrupts(true);
        interrupt_handler.interrupt_request.set_register(0xFF);

        assert_eq!(
            interrupt_handler.fetch_interrupt().unwrap(),
            Interrupt::VBlank
        );
        interrupt_handler.service_interrupt(Interrupt::VBlank);
        interrupt_handler.toggle_interrupts(true);

        assert_eq!(
            interrupt_handler.fetch_interrupt().unwrap(),
            Interrupt::LCDCStat
        );
        interrupt_handler.service_interrupt(Interrupt::LCDCStat);
        interrupt_handler.toggle_interrupts(true);

        assert_eq!(
            interrupt_handler.fetch_interrupt().unwrap(),
            Interrupt::Timer
        );
        interrupt_handler.service_interrupt(Interrupt::Timer);
        interrupt_handler.toggle_interrupts(true);

        assert_eq!(
            interrupt_handler.fetch_interrupt().unwrap(),
            Interrupt::Serial
        );
        interrupt_handler.service_interrupt(Interrupt::Serial);
        interrupt_handler.toggle_interrupts(true);

        assert_eq!(
            interrupt_handler.fetch_interrupt().unwrap(),
            Interrupt::Keypad
        );
        interrupt_handler.service_interrupt(Interrupt::Keypad);
        interrupt_handler.toggle_interrupts(true);

        assert!(interrupt_handler.fetch_interrupt().is_none());
    }
}
