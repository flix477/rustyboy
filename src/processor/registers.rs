use processor::register::*;
use processor::flag_register::FlagRegister;
use processor::program_counter::ProgramCounter;
use processor::stack_pointer::StackPointer;

pub struct Registers {
    pub af: FlagRegister, // accumulator and flags
    pub bc: DualRegister,
    pub de: DualRegister,
    pub hl: DualRegister,
    pub stack_pointer: StackPointer,
    pub program_counter: ProgramCounter
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            af: FlagRegister::new(),
            bc: DualRegister::new(),
            de: DualRegister::new(),
            hl: DualRegister::new(),
            stack_pointer: StackPointer::new(),
            program_counter: ProgramCounter::new()
        };
    }

    pub fn dual_reg(&self, register: RegisterType) -> &DualRegister {
        return match register {
            RegisterType::AF => &self.af.register,
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
                self.bc.high.set(value);
            },
            RegisterType::C => {
                self.bc.low.set(value);
            },
            RegisterType::BC => {
                self.bc.set(value);
            },
            RegisterType::D => {
                self.de.high.set(value);
            },
            RegisterType::E => {
                self.de.low.set(value);
            },
            RegisterType::DE => {
                self.bc.set(value);
            },
            RegisterType::H => {
                self.hl.high.set(value);
            },
            RegisterType::L => {
                self.hl.low.set(value);
            },
            RegisterType::HL => {
                self.hl.set(value);
            },
            _ => {}
        }
    }

    pub fn increment_reg(&mut self, register: RegisterType) {
        match register {
            RegisterType::A => {
                self.af.register.high.increment();
            },
            RegisterType::F => {
                self.af.register.low.increment();
            },
            RegisterType::AF => {
                self.af.register.increment();
            },
            RegisterType::B => {
                self.bc.high.increment();
            },
            RegisterType::C => {
                self.bc.low.increment();
            },
            RegisterType::BC => {
                self.bc.increment();
            },
            RegisterType::D => {
                self.de.high.increment();
            },
            RegisterType::E => {
                self.de.low.increment();
            },
            RegisterType::DE => {
                self.bc.increment();
            },
            RegisterType::H => {
                self.hl.high.increment();
            },
            RegisterType::L => {
                self.hl.low.increment();
            },
            RegisterType::HL => {
                self.hl.increment();
            },
            _ => {}
        }
    }

    pub fn decrement_reg(&mut self, register: RegisterType) {
        match register {
            RegisterType::A => {
                self.af.register.high.decrement();
            },
            RegisterType::F => {
                self.af.register.low.decrement();
            },
            RegisterType::AF => {
                self.af.register.decrement();
            },
            RegisterType::B => {
                self.bc.high.decrement();
            },
            RegisterType::C => {
                self.bc.low.decrement();
            },
            RegisterType::BC => {
                self.bc.decrement();
            },
            RegisterType::D => {
                self.de.high.decrement();
            },
            RegisterType::E => {
                self.de.low.decrement();
            },
            RegisterType::DE => {
                self.bc.decrement();
            },
            RegisterType::H => {
                self.hl.high.decrement();
            },
            RegisterType::L => {
                self.hl.low.decrement();
            },
            RegisterType::HL => {
                self.hl.decrement();
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