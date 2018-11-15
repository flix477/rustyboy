use memory::Memory;
use processor::flag_register::Flag;
use processor::instruction::*;
use processor::decoder::Decoder;
use processor::lr35902::LR35902;
use processor::registers::{Registers, RegisterType};
use processor::register::Register;

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
        let opcode = self.get_immediate();
        if let Some(instruction) = Decoder::decode_opcode(opcode) {
            self.execute(instruction);
        }
    }

    fn get_immediate(&mut self) -> u8 {
        self.registers.program_counter.fetch(&self.memory)
    }

    fn get_immediate16(&mut self) -> u16 {
        (self.get_immediate() as u16) | ((self.get_immediate() as u16) << 8)
    }

    fn push_stack(&mut self, value: u8) {
        self.registers.stack_pointer.push(&mut self.memory, value);
    }

    fn pop_stack(&mut self) -> u8 {
        self.registers.stack_pointer.pop(&self.memory)
    }
}

impl LR35902 for Processor {
    fn ld(&mut self, op1: Operand, op2: Operand) {
        match (op1, op2) {
            (Operand::Immediate16, Operand::Register(r)) => {
                if r.is16bit() {
                    self.ld_n16r16(r);
                } else {
                    self.ld_n16r(r)
                }
            },
            (Operand::Register(r), Operand::Immediate16) => {
                if r.is16bit() {
                    self.ld_r16n16(r);
                } else {
                    self.ld_rn16(r);
                }
            },
            (Operand::Register(r), Operand::Immediate) => {
                if r.is16bit() {
                    self.ld_r16n(r);
                } else {
                    self.ld_rn(r)
                }
            },
            (Operand::Register(r1), Operand::Register(r2)) => {
                if r1.is16bit() && r2.is16bit() {
                    self.ld_r16r16(r1, r2);
                } else if r1.is16bit() && !r2.is16bit() {
                    self.ld_r16r(r1, r2)
                } else if !r1.is16bit() && r2.is16bit() {
                    self.ld_rr16(r1, r2)
                } else {
                    self.ld_rr(r1, r2)
                }
            },
            (Operand::Register(r1), Operand::IncrementedRegister(r2)) => {
                self.ld_rri(r1, r2);
            },
            (Operand::IncrementedRegister(r1), Operand::Register(r2)) => {
                self.ld_rir(r1, r2)
            },
            (Operand::IncrementedImmediate, Operand::Register(r)) => {
                self.ld_nir(r);
            },
            (Operand::Register(r), Operand::IncrementedImmediate) => {
                self.ld_rni(r);
            },
            _ => panic!("bad LD arguments")
        }
    }

    fn ldd(&mut self, op1: Operand, op2: Operand) {
        match (op1, op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                if !r1.is16bit() && r2.is16bit() {
                    self.ld_rr16(r1, r2);
                    self.registers.decrement_reg(r2);
                } else if r1.is16bit() && !r2.is16bit() {
                    self.ld_r16r(r1, r2);
                    self.registers.decrement_reg(r1);
                }
            },
            _ => {}
        }
    }

    fn ldi(&mut self, op1: Operand, op2: Operand) {
        match (op1, op2) {
            (Operand::Register(r1), Operand::Register(r2)) => {
                if !r1.is16bit() && r2.is16bit() {
                    self.ld_rr16(r1, r2);
                    self.registers.increment_reg(r2);
                } else if r1.is16bit() && !r2.is16bit() {
                    self.ld_r16r(r1, r2);
                    self.registers.increment_reg(r1);
                }
            },
            _ => {}
        }
    }

    // writes SP + n to HL
    fn ldhl(&mut self) {
        let n = self.get_immediate() as i8;
        let value = (self.registers.stack_pointer.get() as i32 + n as i32) as u32;
        self.registers.af.set_flags(0);
        self.registers.af.set_flag(Flag::HalfCarry, value > 0xFFF);
        self.registers.af.set_flag(Flag::Carry, value > 0xFFFF);
        self.registers.hl.set(value as u16);
    }

    fn push(&mut self, register: RegisterType) {
        // todo: refactor this, very ugly
        let mut high = 0;
        let mut low = 0;
        {
            let register = self.registers.dual_reg(register);
            high = register.high.get() as u8;
            low = register.low.get() as u8;
        }
        self.push_stack(high);
        self.push_stack(low);
    }

    fn pop(&mut self, register: RegisterType) {
        let low = self.pop_stack() as u16;
        let high = self.pop_stack() as u16;
        self.registers.set_reg(register, (high << 8) | low);
    }

    fn add(&mut self, register: RegisterType, op: Operand) {
        let value2 = match op {
            Operand::Register(RegisterType::HL) => {
                let address = self.registers.hl.get();
                self.memory.get(address) as u16
            },
            Operand::Immediate16 => {
                let address = self.get_immediate16();
                self.memory.get(address) as u16
            }
            Operand::Register(r) => {
                self.registers.reg(r).get()
            },
            _ => {
                panic!("bad arguments for ADD");
            }
        };
        self.add_generic(register, value2);
    }

    fn adc(&mut self, register: RegisterType, op: Operand) {
        let value2 = match op {
            Operand::Register(RegisterType::HL) => {
                let address = self.registers.hl.get();
                self.memory.get(address) as u16
            },
            Operand::Immediate16 => {
                let address = self.get_immediate16();
                self.memory.get(address) as u16
            }
            Operand::Register(r) => {
                self.registers.reg(r).get()
            },
            _ => {
                panic!("bad arguments for ADD");
            }
        };
        let carry = self.registers.af.flag(Flag::Carry);
        self.add_generic(register, value2 + carry as u16);
    }

    fn sub(&mut self, op: Operand) {
        let value1 = self.registers.af.accumulator().get();
        let value2 = match op {
            Operand::Register(RegisterType::HL) => {
                let address = self.registers.hl.get();
                self.memory.get(address) as u16
            },
            Operand::Immediate16 => {
                let address = self.get_immediate16();
                self.memory.get(address) as u16
            }
            Operand::Register(r) => {
                self.registers.reg(r).get()
            },
            _ => {
                panic!("bad arguments for ADD");
            }
        };

        let result = value1 - value2;

        self.registers.af.set_flag(Flag::AddSub, true);
        self.registers.af.set_zero_from_result(result as u8);
        self.registers.af.set_flag(Flag::HalfCarry, true);
        self.registers.af.set_flag(Flag::Carry, true);
    }
}

/**
    functions legend:
        - a: address
        - v: value
        - n: immediate value
        - r: register
        - 16: previous char meaning but 16bit instead of 8
        - i: previous char meaning but +0xFF00
*/
// LD functions
impl Processor {
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

    // writes a 16bit immediate value in a 16bit register
    fn ld_r16n16(&mut self, register: RegisterType) {
        let value = self.get_immediate16();
        self.ld_rv(register, value);
    }

    // writes the value of a register at an immediate address
    fn ld_n16r(&mut self, register: RegisterType) {
        let address = self.get_immediate16();
        self.ld_ar(address, register);
    }

    fn ld_ar16(&mut self, address: u16, register: RegisterType) {
        // TODO: refactor this, completely ugly
        let mut low = 0;
        let mut high = 0;
        {
            let register = self.registers.dual_reg(register);
            low = register.low.get() as u8;
            high = register.high.get() as u8;
        }
        self.ld_av(address, low);
        self.ld_av(address + 1, high);
    }

    // writes the value of a 16bit register at an immediate address
    fn ld_n16r16(&mut self, register: RegisterType) {
        let address = self.get_immediate16();
        self.ld_ar16(address, register);
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

    // puts r2 in r1, 16bit
    fn ld_r16r16(&mut self, r1: RegisterType, r2: RegisterType) {
        let value = self.registers.dual_reg(r2).get();
        self.ld_rv(r1, value);
    }

    // writes the value at the memory address in r2 (a 16bit reg) in r1
    fn ld_rr16(&mut self, r1: RegisterType, r2: RegisterType) {
        let address = self.registers.dual_reg(r2).get();
        self.ld_ra(r1, address);
    }
    // Put memory address $FF00+n into r
    fn ld_rni(&mut self, register: RegisterType) {
        let address = self.get_immediate() as u16 + 0xFF00;
        self.ld_ra(register, address);
    }

    // Put the value in a register into memory address $FF00+n
    fn ld_nir(&mut self, register: RegisterType) {
        let address = self.get_immediate() as u16 + 0xFF00;
        self.ld_ar(address, register);
    }

    // Put value at address $FF00 + r2 into r1
    fn ld_rri(&mut self, r1: RegisterType, r2: RegisterType) {
        let address = self.registers.reg(r2).get() + 0xFF00;
        self.ld_ra(r1, address);
    }

    // Put r2 into address $FF00 + r1
    fn ld_rir(&mut self, r1: RegisterType, r2: RegisterType) {
        let address = self.registers.reg(r1).get() + 0xFF00;
        self.ld_ar(address, r2);
    }
}

impl Processor {
    fn add_generic(&mut self, register: RegisterType, value2: u16) {
        let reg_value = self.registers.reg(register).get();
        let result = reg_value + value2;
        self.registers.set_reg(register, result);
        self.registers.af.set_flag(Flag::AddSub, false);
        self.registers.af.set_flag(Flag::HalfCarry, result > 0xF);
        self.registers.af.set_flag(Flag::Carry, result > 0xFF);
        self.registers.af.set_zero_from_result(result as u8);
    }
}