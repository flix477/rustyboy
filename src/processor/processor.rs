use memory::Memory;
use processor::register::{SingleRegister, DualRegister};
use processor::flag_register::FlagRegister;
use processor::instruction::*;
use processor::decoder::Decoder;
use processor::lr35902::LR35902;
use processor::registers::{Registers, RegisterType};

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

    fn execute(&mut self, instruction: InstructionInfo) {
        match *instruction.mnemonic() {
            InstructionMnemonic::LD => self.ld(instruction),
            _ => {}
        }
    }

    fn step(&mut self) {
        let opcode = self.get_immediate();
        if let Some(instruction) = Decoder::decode_opcode(opcode) {
            self.execute(instruction);
        }
    }

    fn get_immediate(&mut self) -> u8 {
        self.registers.program_counter.fetch(&self.memory)
    }

    fn get_immediate16(&mut self) -> u16 {
        let mut temp_reg = DualRegister::new();
        temp_reg.low.set(self.get_immediate());
        temp_reg.high.set(self.get_immediate());
        temp_reg.get()
    }
}

impl LR35902 for Processor {
    fn ld(&mut self, instruction: InstructionInfo) {
        if let Some(operands) = instruction.operands() {
            match (operands[0], operands[1]) {
                (Operand::Immediate16, Operand::Register(r)) => {
                    self.ld_n16r(r)
                },
                (Operand::Register(r), Operand::Immediate16) => {
                    self.ld_rn16(r)
                },
                (Operand::Register(r), Operand::Immediate) => {
                    if r.is16bit() {
                        self.ld_r16n(r);
                    } else {
                        self.ld_rn(r)
                    }
                },
                (Operand::Register(r1), Operand::Register(r2)) => {
                    if r1.is16bit() && !r2.is16bit() {
                        self.ld_r16r(r1, r2)
                    } else if !r1.is16bit() && r2.is16bit() {
                        self.ld_rr16(r1, r2)
                    } else {
                        self.ld_rr(r1, r2)
                    }
                },
                (Operand::Register(r1), Operand::Address((r2, a))) => {
                    let address = self.registers.reg(r2).get() as u16 + a;
                    self.ld_ra(r1, address);
                },
                (Operand::Address((r2, a)), Operand::Register(r1)) => {
                    let address = self.registers.reg(r2).get() as u16 + a;
                    self.ld_ar(address, r1);
                },
                _ => panic!("bad LD arguments")
            }
        }
    }
}

/**
    LD functions legend:
        - a: address
        - v: value
        - n: immediate value
        - r: register
        - 16: previous char meaning but 16bit instead of 8
*/
impl Processor {
    /**

        LD Instructions

    */
    // writes a value to memory
    fn ld_av(&mut self, address: u16, value: u8) {
        self.memory.set(address, value);
    }

    // writes a value to a register
    fn ld_rv(&mut self, register: RegisterType, value: u16) {
        self.registers.set_reg(register, value);
    }

    // writes the value of a register to memory
    fn ld_ar(&mut self, address: u16, register: RegisterType) {
        let value = self.registers.reg(register).get();
        self.ld_av(address, value as u8);
    }

    // writes the value at a memory address to a register
    fn ld_ra(&mut self, register: RegisterType, address: u16) {
        let value = self.memory.get(address) as u16;
        self.ld_rv(register, value);
    }

    // writes an immediate value to a register
    fn ld_rn(&mut self, register: RegisterType) {
        let value = self.get_immediate() as u16;
        self.ld_rv(register, value);
    }

    // writes the value of an immediate address in a register
    fn ld_rn16(&mut self, register: RegisterType) {
        let address = self.get_immediate16();
        self.ld_ra(register, address);
    }

    // writes the value of a register at an immediate address
    fn ld_n16r(&mut self, register: RegisterType) {
        let address = self.get_immediate16();
        self.ld_ar(address, register);
    }

    // writes an immediate value at the memory address in a register
    fn ld_r16n(&mut self, register: RegisterType) {
        let address = self.registers.dual_reg(register).get();
        let value = self.get_immediate();
        self.ld_av(address, value);
    }

    // puts r2 in r1
    fn ld_rr(&mut self, r1: RegisterType, r2: RegisterType) {
        let r2 = self.registers.reg(r2).get() as u16;
        self.ld_rv(r1, r2);
    }

    // writes the value of r2 at the memory address in r1 (a 16bit reg)
    fn ld_r16r(&mut self, r1: RegisterType, r2: RegisterType) {
        let address = self.registers.dual_reg(r1).get();
        self.ld_ar(address, r2);
    }

    // writes the value at the memory address in r2 (a 16bit reg) in r1
    fn ld_rr16(&mut self, r1: RegisterType, r2: RegisterType) {
        let address = self.registers.dual_reg(r2).get();
        self.ld_ra(r1, address);
    }
}