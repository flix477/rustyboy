use self::flag_register::FlagRegister;
use self::program_counter::ProgramCounter;
use self::register::*;
use self::stack_pointer::StackPointer;
use crate::util::savestate::{read_savestate_byte, read_savestate_u16, write_savestate_u16, LoadSavestateError, Savestate, SavestateStream};

pub mod flag_register;
pub mod program_counter;
pub mod register;
pub mod stack_pointer;

pub const DEFAULT_BC: u16 = 0x13;
pub const DEFAULT_DE: u16 = 0xD8;
pub const DEFAULT_HL: u16 = 0x14D;

#[derive(Copy, Clone)]
pub struct Registers {
    pub af: FlagRegister,
    pub bc: DualRegister,
    pub de: DualRegister,
    pub hl: DualRegister,
    pub stack_pointer: StackPointer,
    pub program_counter: ProgramCounter,
}

impl Registers {
    pub fn new() -> Registers {
        Registers::default()
    }

    pub fn reg(&self, register: RegisterType) -> u16 {
        match register {
            RegisterType::AF => self.af.register.get(),
            RegisterType::BC => self.bc.get(),
            RegisterType::DE => self.de.get(),
            RegisterType::HL => self.hl.get(),
            RegisterType::SP => self.stack_pointer.get(),
            RegisterType::PC => self.program_counter.get(),
            RegisterType::A => self.af.accumulator().get(),
            RegisterType::F => self.af.flags().get(),
            RegisterType::B => self.bc.high.get(),
            RegisterType::C => self.bc.low.get(),
            RegisterType::D => self.de.high.get(),
            RegisterType::E => self.de.low.get(),
            RegisterType::H => self.hl.high.get(),
            RegisterType::L => self.hl.low.get(),
        }
    }

    pub fn set_reg(&mut self, register: RegisterType, value: u16) {
        match register {
            RegisterType::A => {
                self.af.set_accumulator(value as u8);
            }
            RegisterType::F => {
                self.af.set_flags(value as u8);
            }
            RegisterType::AF => {
                self.af.set(value);
            }
            RegisterType::B => {
                self.bc.high.set(value);
            }
            RegisterType::C => {
                self.bc.low.set(value);
            }
            RegisterType::BC => {
                self.bc.set(value);
            }
            RegisterType::D => {
                self.de.high.set(value);
            }
            RegisterType::E => {
                self.de.low.set(value);
            }
            RegisterType::DE => {
                self.de.set(value);
            }
            RegisterType::H => {
                self.hl.high.set(value);
            }
            RegisterType::L => {
                self.hl.low.set(value);
            }
            RegisterType::HL => {
                self.hl.set(value);
            }
            RegisterType::PC => self.program_counter.set(value),
            RegisterType::SP => self.stack_pointer.set(value),
        }
    }
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            af: FlagRegister::new(),
            bc: DualRegister::from(DEFAULT_BC),
            de: DualRegister::from(DEFAULT_DE),
            hl: DualRegister::from(DEFAULT_HL),
            stack_pointer: StackPointer::new(),
            program_counter: ProgramCounter::default(),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegisterType {
    AF,
    A,
    F,
    BC,
    B,
    C,
    DE,
    D,
    E,
    HL,
    H,
    L,
    SP,
    PC,
}

impl ToString for RegisterType {
    fn to_string(&self) -> String {
        match self {
            RegisterType::A => "a".to_string(),
            RegisterType::F => "f".to_string(),
            RegisterType::AF => "af".to_string(),
            RegisterType::B => "b".to_string(),
            RegisterType::C => "c".to_string(),
            RegisterType::BC => "bc".to_string(),
            RegisterType::D => "d".to_string(),
            RegisterType::E => "e".to_string(),
            RegisterType::DE => "de".to_string(),
            RegisterType::H => "h".to_string(),
            RegisterType::L => "l".to_string(),
            RegisterType::HL => "hl".to_string(),
            RegisterType::SP => "sp".to_string(),
            RegisterType::PC => "pc".to_string(),
        }
    }
}

impl RegisterType {
    pub fn is16bit(self) -> bool {
        self == RegisterType::AF
            || self == RegisterType::BC
            || self == RegisterType::DE
            || self == RegisterType::HL
            || self == RegisterType::SP
            || self == RegisterType::PC
    }
}

impl Savestate for Registers {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.af.register.high.value);
        buffer.push(self.af.register.low.value);
        buffer.push(self.bc.high.value);
        buffer.push(self.bc.low.value);
        buffer.push(self.de.high.value);
        buffer.push(self.de.low.value);
        buffer.push(self.hl.high.value);
        buffer.push(self.hl.low.value);
        write_savestate_u16(buffer, self.stack_pointer.value);
        write_savestate_u16(buffer, self.program_counter.value);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream<'a>,
    ) -> Result<(), LoadSavestateError> {
        self.af.register.high.value = read_savestate_byte(buffer)?;
        self.af.register.low.value = read_savestate_byte(buffer)?;
        self.bc.high.value = read_savestate_byte(buffer)?;
        self.bc.low.value = read_savestate_byte(buffer)?;
        self.de.high.value = read_savestate_byte(buffer)?;
        self.de.low.value = read_savestate_byte(buffer)?;
        self.hl.high.value = read_savestate_byte(buffer)?;
        self.hl.low.value = read_savestate_byte(buffer)?;
        self.stack_pointer.value = read_savestate_u16(buffer)?;
        self.program_counter.value = read_savestate_u16(buffer)?;
        Ok(())
    }
}
