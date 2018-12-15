mod register;
mod flag_register;
mod instruction;
mod decoder;
mod lr35902;
mod registers;
mod program_counter;
mod stack_pointer;
pub mod interrupt;
use crate::processor::flag_register::Flag;
use crate::processor::decoder::Decoder;
use crate::processor::lr35902::LR35902;
use crate::processor::registers::{Registers, RegisterType};
use crate::processor::instruction::Prefix;
use crate::bus::Bus;
use crate::util::bitflags::Bitflags;
use crate::processor::register::Register;
use crate::processor::instruction::Mnemonic;

const CLOCK_FREQUENCY: f64 = 4194304.0; // Hz

pub struct Processor {
    registers: Registers,
    clock_frequency: f64,
    leftover_time: f64,
    last_instruction_cycles: u8,
    stopped: bool
}

impl Processor {
    pub fn new() -> Processor {
        return Processor {
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY,
            leftover_time: 0.0,
            last_instruction_cycles: 0,
            stopped: false
        };
    }

    pub fn update<H: Bus>(&mut self, bus: &mut H, delta: f64) {
        if !self.stopped {
            self.leftover_time += delta;
            while
                !self.stopped &&
                (self.last_instruction_cycles == 0||
                self.leftover_time >= (self.last_instruction_cycles as f64 / CLOCK_FREQUENCY))
            {
                self.leftover_time -= if self.last_instruction_cycles > 0 {
                    self.last_instruction_cycles as f64 / CLOCK_FREQUENCY
                } else { self.leftover_time };
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
        let opcode = self.immediate(bus);
        if let Some(instruction) = Decoder::decode_opcode(opcode, prefix) {
            let cycle_count = instruction.cycle_count();
            if let Err(err) = self.execute(bus, instruction) {
                println!("Error with instruction: {:?}", err);
                panic!()
            }
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