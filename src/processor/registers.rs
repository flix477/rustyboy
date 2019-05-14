use crate::processor::flag_register::FlagRegister;
use crate::processor::program_counter::ProgramCounter;
use crate::processor::register::*;
use crate::processor::stack_pointer::StackPointer;
use std::fmt::{Debug, Error, Formatter};

pub const DEFAULT_BC: u16 = 0;
pub const DEFAULT_DE: u16 = 0xFF56;
pub const DEFAULT_HL: u16 = 0xD;

pub struct Registers {
    pub af: FlagRegister,
    pub bc: DualRegister,
    pub de: DualRegister,
    pub hl: DualRegister,
    pub stack_pointer: StackPointer,
    pub program_counter: ProgramCounter,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "AF: 0x{:X}\nBC: 0x{:X}\nDE: 0x{:X}\nHL: 0x{:X}\nSP: 0x{:X}\nPC: 0x{:X}",
            self.reg(RegisterType::AF),
            self.reg(RegisterType::BC),
            self.reg(RegisterType::DE),
            self.reg(RegisterType::HL),
            self.reg(RegisterType::SP),
            self.reg(RegisterType::PC)
        )
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: FlagRegister::new(),
            bc: DualRegister::from(DEFAULT_BC),
            de: DualRegister::from(DEFAULT_DE),
            hl: DualRegister::from(DEFAULT_HL),
            stack_pointer: StackPointer::new(),
            program_counter: ProgramCounter::new(),
        }
    }

    pub fn reg(&self, register: RegisterType) -> u16 {
        match register {
            RegisterType::AF => self.af.register().get(),
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
    pub fn is16bit(&self) -> bool {
        *self == RegisterType::AF
            || *self == RegisterType::BC
            || *self == RegisterType::DE
            || *self == RegisterType::HL
            || *self == RegisterType::SP
            || *self == RegisterType::PC
    }
}
