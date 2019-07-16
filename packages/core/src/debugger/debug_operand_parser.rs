use crate::bus::Readable;
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::{
    flag_register::Flag, program_counter::ProgramCounter, RegisterType,
};
use crate::util::bitflags::Bitflags;
use crate::debugger::debug_info::ProcessorDebugInfo;

pub struct DebugOperandParser<'a> {
    program_counter: ProgramCounter,
    cpu_debug_info: &'a ProcessorDebugInfo,
}

impl<'a> DebugOperandParser<'a> {
    pub fn new(start_address: u16, cpu_debug_info: &'a ProcessorDebugInfo) -> Self {
        Self {
            program_counter: ProgramCounter::new(start_address),
            cpu_debug_info,
        }
    }

    pub fn program_counter(&self) -> &ProgramCounter {
        &self.program_counter
    }
}

impl OperandParser for DebugOperandParser<'_> {
    fn mut_program_counter(&mut self) -> &mut ProgramCounter {
        &mut self.program_counter
    }

    fn reg(&self, register: RegisterType) -> u16 {
        self.cpu_debug_info.registers.reg(register)
    }

    fn flag(&self, flag: Flag) -> bool {
        self.cpu_debug_info.registers.af.flag(flag)
    }
}

pub struct ReadableVec<'a> {
    pub value: &'a Vec<u8>,
}

impl Readable for ReadableVec<'_> {
    fn read(&self, address: u16) -> u8 {
        self.value[address as usize]
    }
}
