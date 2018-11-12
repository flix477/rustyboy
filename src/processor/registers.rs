use processor::register::*;
use processor::flag_register::FlagRegister;
use processor::program_counter::ProgramCounter;

pub struct Registers {
    pub af: FlagRegister, // accumulator and flags
    pub bc: DualRegister,
    pub de: DualRegister,
    pub hl: DualRegister,
    pub stack_pointer: u16,
    pub program_counter: ProgramCounter
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            af: FlagRegister::new(),
            bc: DualRegister::new(),
            de: DualRegister::new(),
            hl: DualRegister::new(),
            stack_pointer: 0,
            program_counter: ProgramCounter::new()
        };
    }

    pub fn dual_reg(&self, register: RegisterType) -> &DualRegister {
        return match register {
//            RegisterType::AF => self.af.register(),
            RegisterType::BC => &self.bc,
            RegisterType::DE => &self.de,
            RegisterType::HL => &self.hl,
            _ => panic!("not a dual register")
        }
    }

    pub fn reg(&self, register: RegisterType) -> &SingleRegister {
        return match register {
            RegisterType::A => &self.af.accumulator(),
            RegisterType::F => &self.af.flags(),
            RegisterType::B => &self.bc.high,
            RegisterType::C => &self.bc.low,
            RegisterType::D => &self.de.high,
            RegisterType::E => &self.de.low,
            RegisterType::H => &self.hl.high,
            RegisterType::L => &self.hl.low,
            _ => panic!("not a single register")
        }
    }

    pub fn set_reg(&mut self, register: RegisterType, value: u16) {
        match register {
            RegisterType::A => {
                self.af.set_accumulator(value as u8);
            },
            RegisterType::F => {
                self.af.set_flags(value as u8);
            },
            RegisterType::AF => {
                self.af.register.set(value);
            },
            RegisterType::B => {
                self.bc.high.set(value as u8);
            },
            RegisterType::C => {
                self.bc.low.set(value as u8);
            },
            RegisterType::BC => {
                self.bc.set(value);
            },
            RegisterType::D => {
                self.de.high.set(value as u8);
            },
            RegisterType::E => {
                self.de.low.set(value as u8);
            },
            RegisterType::BC => {
                self.bc.set(value);
            },
            RegisterType::H => {
                self.hl.high.set(value as u8);
            },
            RegisterType::L => {
                self.hl.low.set(value as u8);
            },
            RegisterType::HL => {
                self.hl.set(value);
            },
            _ => {}
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
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
    PC
}

impl RegisterType {
    pub fn is16bit(&self) -> bool {
        return
            *self == RegisterType::AF ||
                *self == RegisterType::BC ||
                *self == RegisterType::DE ||
                *self == RegisterType::HL ||
                *self == RegisterType::SP ||
                *self == RegisterType::PC
        ;
    }
}