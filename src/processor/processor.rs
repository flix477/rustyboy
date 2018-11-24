use memory::Memory;
use processor::flag_register::Flag;
use processor::decoder::Decoder;
use processor::lr35902::LR35902;
use processor::registers::{Registers, RegisterType};
use processor::instruction::Prefix;

const CLOCK_FREQUENCY: f64 = 4.194304; // MHz

pub struct Processor {
    memory: Memory,
    registers: Registers,
    clock_frequency: f64
}

impl Processor {
    pub fn new(memory: Memory) -> Processor {
        return Processor {
            memory,
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY
        };
    }

    fn step(&mut self) {
        self.execute_next(Prefix::None)
    }
}

impl LR35902 for Processor {
    fn immediate(&mut self) -> u8 {
        self.registers.program_counter.fetch(&self.memory)
    }

    fn immediate16(&mut self) -> u16 {
        (self.immediate() as u16) | ((self.immediate() as u16) << 8)
    }

    fn reg(&self, register: RegisterType) -> u16 {
        self.registers.reg(register)
    }

    fn set_reg(&mut self, register: RegisterType, value: u16) {
        self.registers.set_reg(register, value);
    }

    fn address(&self, address: u16) -> u8 {
        self.memory.get(address)
    }

    fn set_address(&mut self, address: u16, value: u8) {
        self.memory.set(address, value);
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

    fn push_stack(&mut self, value: u16) {
        self.registers.stack_pointer.push(&mut self.memory, value);
    }

    fn pop_stack(&mut self) -> u16 {
        self.registers.stack_pointer.pop(&self.memory)
    }

    fn execute_next(&mut self, prefix: Prefix) {
        let opcode = self.immediate();
        if let Some(instruction) = Decoder::decode_opcode(opcode, prefix) {
            self.execute(instruction);
        }
    }
}
