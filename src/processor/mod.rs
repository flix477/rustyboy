mod register;
mod flag_register;
mod instruction;
mod decoder;
mod lr35902;
mod registers;
mod program_counter;
mod stack_pointer;

use processor::flag_register::Flag;
use processor::decoder::Decoder;
use processor::lr35902::LR35902;
use processor::registers::{Registers, RegisterType};
use processor::instruction::Prefix;
use bus::Bus;

const CLOCK_FREQUENCY: f64 = 4.194304; // MHz

pub struct Processor {
    registers: Registers,
    clock_frequency: f64
}

impl Processor {
    pub fn new() -> Processor {
        return Processor {
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY
        };
    }

    pub fn step<H: Bus>(&mut self, bus: &mut H) {
        self.execute_next(bus, Prefix::None)
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

    fn set_zero_from_result(&mut self, result: u8) {
        self.registers.af.set_zero_from_result(result);
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
            self.execute(bus, instruction);
        }
    }
}


//
//use self::decoder::Decoder;
//use self::processor::Processor;
//use self::super::memory::Memory;
//use self::lr35902::LR35902;
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn implemented_coverage() {
//        let mut cpu = Processor::new(Memory::new());
//        let mut covered = Vec::new();
//        for i in 0..0xff {
//            if let Some(instruction) = Decoder::decode_opcode(i) {
//                if let Ok(_) = cpu.execute(instruction) {
//                    covered.push(i);
//                }
//            }
//        }
//        assert_eq!(covered.len(), 0xff - 11);
//    }
//}