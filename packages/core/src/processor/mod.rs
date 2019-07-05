pub mod decoder;
pub mod instruction;
pub mod interrupt;
pub mod lr35902;
pub mod operand_parser;
#[cfg(test)]
mod processor_tests;
pub mod registers;

use self::decoder::Decoder;
use self::instruction::Prefix;
use self::lr35902::LR35902;
use self::registers::flag_register::Flag;
use self::registers::register::Register;
use self::registers::{RegisterType, Registers};
use crate::bus::Bus;
use crate::debugger::debug_info::ProcessorDebugInfo;
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::program_counter::ProgramCounter;
use crate::util::bitflags::Bitflags;

pub struct Processor {
    registers: Registers,
    halt_mode: HaltMode,
    cycles_left: u8,
    pending_ei: bool,
}

impl Default for Processor {
    fn default() -> Self {
        Self {
            registers: Registers::new(),
            halt_mode: HaltMode::None,
            cycles_left: 0,
            pending_ei: false,
        }
    }
}

impl Processor {
    pub fn new() -> Processor {
        Self::default()
    }

    pub fn step<H: Bus>(&mut self, bus: &mut H) -> ProcessorStepResult {
        let interrupt = bus.fetch_interrupt();
        if let Some(interrupt) = interrupt {
            self.halt_mode = HaltMode::None;
            if bus.master_interrupt_enable() {
                bus.service_interrupt(interrupt);
                let pc = self.registers.program_counter.get();
                self.push_stack(bus, pc);
                self.jp(interrupt.address());
            }
        }

        if self.cycles_left == 0 && self.halt_mode != HaltMode::Normal {
            if self.pending_ei {
                self.immediate_ei(bus);
            }

            let pc = self.registers.program_counter.get();
            self.cycles_left = self.execute_next(bus, Prefix::None);

            // emulate this one weird bug
            // https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf section 4.10
            if self.halt_mode == HaltMode::Bugged {
                self.halt_mode = HaltMode::None;
                self.registers.program_counter.set(pc);
                self.cycles_left += self.execute_next(bus, Prefix::None);
            }
        } else {
            self.cycles_left -= 1;
            if self.cycles_left == 0 {
                return ProcessorStepResult::NewInstruction;
            }
        }

        ProcessorStepResult::CycleCompensation
    }

    pub fn debug_info(&self) -> ProcessorDebugInfo {
        ProcessorDebugInfo {
            registers: self.registers,
        }
    }
}

impl OperandParser for Processor {
    fn mut_program_counter(&mut self) -> &mut ProgramCounter {
        &mut self.registers.program_counter
    }

    fn reg(&self, register: RegisterType) -> u16 {
        self.registers.reg(register)
    }

    fn flag(&self, flag: Flag) -> bool {
        self.registers.af.flag(flag)
    }
}

impl LR35902 for Processor {
    fn set_reg(&mut self, register: RegisterType, value: u16) {
        self.registers.set_reg(register, value);
    }

    fn set_address<H: Bus>(&self, bus: &mut H, address: u16, value: u8) {
        bus.write(address, value);
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

    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) -> u8 {
        let opcode = self.immediate(bus);
        if let Some(instruction) = Decoder::decode_opcode(opcode, prefix) {
            let cycle_count = instruction.cycle_count();
            self.execute(bus, instruction)
                .expect("Error with instruction");
            cycle_count
        } else {
            0 // i guess?
        }
    }

    fn halt<H: Bus>(&mut self, bus: &H) {
        if !bus.master_interrupt_enable() && bus.fetch_interrupt().is_some() {
            self.halt_mode = HaltMode::Bugged;
        } else {
            self.halt_mode = HaltMode::Normal;
        }
    }

    fn stop(&mut self) {
        self.halt_mode = HaltMode::Normal;
    }

    fn ei(&mut self) {
        self.pending_ei = true;
    }

    fn immediate_ei<H: Bus>(&mut self, bus: &mut H) {
        bus.toggle_interrupts(true);
        self.pending_ei = false;
    }
}

#[derive(PartialEq)]
enum HaltMode {
    Normal,
    Bugged,
    None,
}

#[derive(PartialEq)]
pub enum ProcessorStepResult {
    CycleCompensation,
    NewInstruction,
}
