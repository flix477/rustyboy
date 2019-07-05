use super::debug_info::DebugInfo;
use crate::bus::Readable;
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::{
    flag_register::Flag, program_counter::ProgramCounter, RegisterType,
};
use crate::util::bitflags::Bitflags;

pub struct DebugOperandParser<'a> {
    program_counter: ProgramCounter,
    debug_info: &'a DebugInfo,
}

impl<'a> DebugOperandParser<'a> {
    pub fn new(start_address: u16, debug_info: &'a DebugInfo) -> Self {
        Self {
            program_counter: ProgramCounter::new(start_address),
            debug_info,
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
        self.debug_info.cpu_debug_info.registers.reg(register)
    }

    fn flag(&self, flag: Flag) -> bool {
        self.debug_info.cpu_debug_info.registers.af.flag(flag)
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
