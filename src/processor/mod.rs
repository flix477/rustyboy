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
use self::instruction::{AddressType, ValueType};
use crate::bus::Bus;
use crate::config::Config;
use crate::debugger::{DebugInfo, Debugger, DebuggerState};
use crate::processor::decoder::Decoder;
use crate::processor::flag_register::Flag;
use crate::processor::instruction::{InstructionInfo, Prefix};
use crate::processor::lr35902::LR35902;
use crate::processor::register::Register;
use crate::processor::registers::{RegisterType, Registers};
use crate::util::bitflags::Bitflags;

const CLOCK_FREQUENCY: f64 = 4194304.0; // Hz

pub struct Processor {
    registers: Registers,
    clock_frequency: f64,
    leftover_time: f64,
    last_instruction_cycles: u8,
    stopped: bool,
    pub debugger: Option<Debugger>,
}

impl Processor {
    pub fn new(debugger_config: Option<DebuggerState>) -> Processor {
        Processor {
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY,
            leftover_time: 0.0,
            last_instruction_cycles: 0,
            stopped: false,
            debugger: if let Some(state) = debugger_config {
                Some(Debugger::from_state(state))
            } else {
                None
            },
        }
    }

    pub fn update<H: Bus>(&mut self, bus: &mut H, delta: f64) {
        if !self.stopped {
            self.leftover_time += delta;
            while !self.stopped
                && (self.last_instruction_cycles == 0
                    || self.leftover_time
                        >= (self.last_instruction_cycles as f64 / CLOCK_FREQUENCY))
            {
                self.leftover_time -= if self.last_instruction_cycles > 0 {
                    self.last_instruction_cycles as f64 / CLOCK_FREQUENCY
                } else {
                    self.leftover_time
                };
                self.last_instruction_cycles = self.step(bus);
            }
        } else {
            self.step(bus);
        }
    }

    pub fn step<H: Bus>(&mut self, bus: &mut H) -> u8 {
        let interrupt = bus.fetch_interrupt();
        if let Some(interrupt) = interrupt {
            self.stopped = false;
            let pc = self.registers.program_counter.get();
            self.push_stack(bus, pc);
            self.jp(interrupt.address());
            0 // lol TODO
        } else if !self.stopped {
            self.execute_next(bus, Prefix::None)
        } else {
            0
        }
    }

    fn peek<H: Bus>(&self, bus: &H) -> u8 {
        self.registers.program_counter.peek(bus)
    }

    fn peek16<H: Bus>(&self, bus: &H) -> u16 {
        self.registers.program_counter.peek16(bus)
    }

    fn peek_addr_value<H: Bus>(&self, address: AddressType, bus: &H) -> u16 {
        match address {
            AddressType::Register(reg) => self.reg(reg),
            AddressType::IncRegister(reg) => self.reg(reg).wrapping_add(0xFF00),
            AddressType::Immediate => self.peek16(bus),
            AddressType::IncImmediate => (self.peek(bus) as u16).wrapping_add(0xFF00),
        }
    }

    fn peek_value<H: Bus>(&self, value: ValueType, bus: &H) -> u16 {
        match value {
            ValueType::Immediate => self.peek(bus) as u16,
            ValueType::Immediate16 => self.peek16(bus),
            ValueType::Address(address) => self.peek_addr_value(address, bus),
            ValueType::Register(reg) => self.reg(reg),
            ValueType::Constant(constant) => constant,
        }
    }

    fn debugger_check<H: Bus>(&mut self, bus: &H, line: u16, instruction: &InstructionInfo) {
        if let Some(ref mut debugger) = self.debugger {
            if debugger.should_run(line) {
                let debug_info = DebugInfo {
                    registers: &self.registers,
                    line,
                    instruction: &instruction,
                };
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
            return cycle_count;
        }
        0 // i guess lol
    }

    fn halt(&mut self) {
        self.stopped = true;
    }

    fn stop(&mut self) {
        self.stopped = true;
    }
}
