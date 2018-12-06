mod register;
mod flag_register;
mod instruction;
mod decoder;
mod lr35902;
mod registers;
mod program_counter;
mod stack_pointer;
pub mod interrupt;

use processor::flag_register::Flag;
use processor::decoder::Decoder;
use processor::lr35902::LR35902;
use processor::registers::{Registers, RegisterType};
use processor::instruction::Prefix;
use bus::Bus;
use util::bitflags::Bitflags;
use processor::register::Register;

const CLOCK_FREQUENCY: f64 = 4.194304; // MHz

pub struct Processor {
    registers: Registers,
    clock_frequency: f64,
}

impl Processor {
    pub fn new() -> Processor {
        return Processor {
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY,
        };
    }

    pub fn step<H: Bus>(&mut self, bus: &mut H) {
        let interrupt = bus.fetch_interrupt();
        if let Some(interrupt) = interrupt {
            let pc = self.registers.program_counter.get();
            self.push_stack(bus, pc);
            self.jp(interrupt.address())
        } else {
            self.execute_next(bus, Prefix::None)
        }
    }
}

impl LR35902 for Processor {
    fn immediate<H: Bus>(&mut self, bus: &H) -> u8 {
        self.registers.program_counter.fetch(bus)
    }

    fn immediate16<H: Bus>(&mut self, bus: &H) -> u16 {
        (self.immediate(bus) as u16) | ((self.immediate(bus) as u16) << 8)
    }

    fn reg(&self, register: RegisterType) -> u16 {
        self.registers.reg(register)
    }

    fn set_reg(&mut self, register: RegisterType, value: u16) {
        self.registers.set_reg(register, value);
    }

    fn address<H: Bus>(&self, bus: &H, address: u16) -> u8 {
        bus.read(address)
    }

    fn set_address<H: Bus>(&self, bus: &mut H, address: u16, value: u8) {
        bus.write(address, value);
    }

    fn flag(&self, flag: Flag) -> bool {
        self.registers.af.flag(flag)
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        self.registers.af.set_flag(flag, value);
    }

    fn push_stack<H: Bus>(&mut self, bus: &mut H, value: u16) {
        self.registers.stack_pointer.push(bus, value);
    }

    fn pop_stack<H: Bus>(&mut self, bus: &mut H) -> u16 {
        self.registers.stack_pointer.pop(bus)
    }

    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) {
        let opcode = self.immediate(bus);
        if let Some(instruction) = Decoder::decode_opcode(opcode, prefix) {
            if let Err(err) = self.execute(bus, instruction) {
                println!("Error with instruction: {:?}", err);
            }
        }
    }
}