pub mod decoder;
pub mod instruction;
pub mod interrupt;
pub mod lr35902;
pub mod operand_parser;
#[cfg(test)]
mod processor_tests;
pub mod registers;

use self::instruction::Prefix;
use self::lr35902::LR35902;
use self::registers::flag_register::Flag;
use self::registers::register::Register;
use self::registers::{RegisterType, Registers};
use crate::bus::Bus;
use crate::processor::decoder::decode_opcode;
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::program_counter::ProgramCounter;
use crate::util::bitflags::Bitflags;
use crate::util::savestate::{
    read_savestate_bool, read_savestate_byte, LoadSavestateError, Savestate,
};

/// This struct contains the logic for the GameBoy's processor
pub struct Processor {
    pub registers: Registers,
    halt_mode: HaltMode,
    /// Cycles left from the last instruction
    cycles_left: u8,
    /// Whether or not an EI instruction was last executed
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

    /// This method performs a single CPU step and returns the result
    pub fn step<H: Bus>(&mut self, bus: &mut H) -> ProcessorStepResult {
        // check for interrupts
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

            // if in bugged HALT mode, execute the last instruction another time
            if self.halt_mode == HaltMode::Bugged {
                self.halt_mode = HaltMode::None;
                self.registers.program_counter.set(pc);
                self.cycles_left += self.execute_next(bus, Prefix::None);
            }
        } else {
            self.cycles_left = self.cycles_left.saturating_sub(1);
            if self.cycles_left == 0 {
                return ProcessorStepResult::InstructionCompleted;
            }
        }

        ProcessorStepResult::InstructionInProgress
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
        if let Some(instruction) = decode_opcode(opcode, prefix) {
            let cycle_count = instruction.cycle_count;
            self.execute(bus, instruction);
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

/// Represents the different modes the HALT instruction puts the CPU through.
/// The HALT instruction is used to stop CPU execution until an interrupt comes in,
/// but when used while the IME register is off and the IF register has pending interrupts,
/// it bugs and repeats the instruction following it twice.
///
/// https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf section 4.10
#[derive(Copy, Clone, Debug, PartialEq)]
enum HaltMode {
    /// Normal HALT mode, CPU execution is stopped
    Normal,
    /// Bugged HALT mode, CPU execution continues and next instruction is executed twice
    Bugged,
    /// No HALT mode, CPU execution continues
    None,
}

impl HaltMode {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(HaltMode::Normal),
            1 => Some(HaltMode::Bugged),
            2 => Some(HaltMode::None),
            _ => None,
        }
    }
}

/// Represents the result of a single CPU step
#[derive(PartialEq)]
pub enum ProcessorStepResult {
    /// Means we only decremented the cycles left from the last instruction
    InstructionInProgress,
    /// Means we have completely finished the last instruction
    InstructionCompleted,
}

impl Savestate for Processor {
    fn dump_savestate(&self, buffer: &mut Vec<u8>) {
        self.registers.dump_savestate(buffer);
        buffer.push(self.halt_mode as u8);
        buffer.push(self.cycles_left);
        buffer.push(self.pending_ei as u8);
    }

    fn load_savestate<'a>(
        &mut self,
        buffer: &mut SavestateStream,
    ) -> Result<(), LoadSavestateError> {
        self.registers.load_savestate(buffer)?;
        self.halt_mode = buffer
            .next()
            .cloned()
            .and_then(HaltMode::from)
            .ok_or(LoadSavestateError::InvalidSavestate)?;
        self.cycles_left = read_savestate_byte(buffer)?;
        self.pending_ei = read_savestate_bool(buffer)?;
        Ok(())
    }
}
