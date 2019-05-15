mod decoder;
mod flag_register;
pub mod instruction;
pub mod interrupt;
pub mod lr35902;
mod processor_tests;
mod program_counter;
mod register;
pub mod registers;
mod stack_pointer;

use crate::bus::Bus;
use crate::debugger::debug_info::DebugInfo;
use crate::debugger::{Debugger, DebuggerState};
use crate::processor::decoder::Decoder;
use crate::processor::flag_register::Flag;
use crate::processor::instruction::Reference::Address;
use crate::processor::instruction::{AddressType, InstructionInfo, Operand, Prefix, Reference};
use crate::processor::lr35902::LR35902;
use crate::processor::register::Register;
use crate::processor::registers::{RegisterType, Registers};
use crate::util::bitflags::Bitflags;

const CLOCK_FREQUENCY: f64 = 4194304.0; // Hz

pub struct Processor {
    registers: Registers,
    clock_frequency: f64,
    halt_mode: HaltMode,
    pub debugger: Option<Debugger>,
    cycles_left: u8,
    pending_ei: bool,
}

impl Processor {
    pub fn new(debugger_config: Option<DebuggerState>) -> Processor {
        Processor {
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY,
            halt_mode: HaltMode::None,
            debugger: if let Some(state) = debugger_config {
                Some(Debugger::from_state(state))
            } else {
                None
            },
            cycles_left: 0,
            pending_ei: false,
        }
    }

    pub fn step<H: Bus>(&mut self, bus: &mut H) {
        let interrupt = bus.fetch_interrupt();
        if let Some(interrupt) = interrupt {
            self.halt_mode = HaltMode::None;
            if bus.master_interrupt_enable() {
                println!("interrupt");
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
        } else if self.cycles_left > 0 {
            self.cycles_left -= 1;
        }
    }

    fn debugger_check<H: Bus>(&mut self, bus: &H, line: u16, instruction: &InstructionInfo) {
        if let Some(ref mut debugger) = self.debugger {
            let debug_info = DebugInfo {
                registers: &self.registers,
                line,
                instruction: &instruction,
            };
            if debugger.should_run(&debug_info) {
                debugger.run(debug_info, bus);
            }
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

    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) -> u8 {
        let line = self.registers.program_counter.get();
        let opcode = self.immediate(bus);
        if let Some(instruction) = Decoder::decode_opcode(opcode, prefix) {
            let cycle_count = instruction.cycle_count();
            self.debugger_check(bus, line, &instruction);
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
    None
}