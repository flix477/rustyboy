use processor::instruction::{Operand, ValueType, AddressType};
use processor::registers::RegisterType;
use processor::instruction::{InstructionInfo, Mnemonic};
use processor::flag_register::Flag;
use processor::instruction::Reference;
use processor::instruction::Prefix;
use util::bits;

pub trait LR35902 {
    fn immediate(&mut self) -> u8;
    fn immediate16(&mut self) -> u16;
    fn reg(&self, register: RegisterType) -> u16;
    fn set_reg(&mut self, register: RegisterType, value: u16);
    fn address(&self, address: u16) -> u8;
    fn set_address(&mut self, address: u16, value: u8);
    fn flag(&self, flag: Flag) -> bool;
    fn set_flag(&mut self, flag: Flag, value: bool);
    fn set_zero_from_result(&mut self, result: u8);
    fn push_stack(&mut self, value: u16);
    fn pop_stack(&mut self) -> u16;
    fn execute_next(&mut self, prefix: Prefix);

    fn execute(&mut self, instruction: InstructionInfo) -> Result<(), &str> {
        let mnemonic = *instruction.mnemonic();
        match mnemonic {
            Mnemonic::LD => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Reference(r), Operand::Value(v)) = (operands[0], operands[1]) {
                        let value = self.operand_value(v);
                        self.ld(r, value);
                    } else {
                        return Err("Wrong arguments");
                    }
                } else {
                    return Err("Requires arguments");
                }
            },
            Mnemonic::LDD |
            Mnemonic::LDI => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Reference(r1), Operand::Reference(r2)) = (operands[0], operands[1]) {
                        match mnemonic {
                            Mnemonic::LDD => self.ldd(r1, r2),
                            Mnemonic::LDI => self.ldi(r1, r2),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong arguments");
                    }
                } else {
                    return Err("Requires arguments");
                }
            }
            Mnemonic::LDHL => self.ldhl(),
            Mnemonic::PUSH => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(value) = operands[0] {
                        let value = self.operand_value(value);
                        self.push(value);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::POP => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(Reference::Register(r)) = operands[0] {
                        self.pop(r);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::ADD => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Reference(Reference::Register(r)), Operand::Value(v)) = (operands[0], operands[1]) {
                        let value = self.operand_value(v);
                        if r.is16bit() {
                            self.add16(r, value);
                        } else {
                            self.add(r, value as u8);
                        }
                    } else {
                        return Err("Wrong arguments");
                    }
                } else {
                    return Err("Requires arguments");
                }
            }
            Mnemonic::ADC |
            Mnemonic::SUB |
            Mnemonic::SBC |
            Mnemonic::AND |
            Mnemonic::OR |
            Mnemonic::XOR => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(v) = operands[0] {
                        let value = self.operand_value(v) as u8;
                        match mnemonic {
                            Mnemonic::ADC => self.adc(value),
                            Mnemonic::SUB => self.sub(value),
                            Mnemonic::SBC => self.sbc(value),
                            Mnemonic::AND => self.and(value),
                            Mnemonic::OR => self.or(value),
                            Mnemonic::XOR => self.xor(value),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong arguments");
                    }
                } else {
                    return Err("Requires arguments");
                }
            },
            Mnemonic::CP => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(value) = operands[0] {
                        let value = self.operand_value(value);
                        self.cp(value as u8);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::INC |
            Mnemonic::DEC => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(reference) = operands[0] {
                        if let Mnemonic::INC = mnemonic {
                            self.inc(reference);
                        } else {
                            self.dec(reference);
                        }
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::DAA => self.daa(),
            Mnemonic::CPL => self.cpl(),
            Mnemonic::CCF => self.ccf(),
            Mnemonic::SCF => self.scf(),
            Mnemonic::NOP => {},
            Mnemonic::HALT => self.halt(),
            Mnemonic::STOP => self.stop(),
            Mnemonic::DI => self.di(),
            Mnemonic::EI => self.ei(),
            Mnemonic::RLC |
            Mnemonic::RL |
            Mnemonic::RRC |
            Mnemonic::RR => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(r) = operands[0] {
                        match mnemonic {
                            Mnemonic::RLC => self.rlc(r),
                            Mnemonic::RL => self.rl(r),
                            Mnemonic::RRC => self.rrc(r),
                            Mnemonic::RR => self.rr(r),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::SLA |
            Mnemonic::SRA |
            Mnemonic::SRL => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(r) = operands[0] {
                        match mnemonic {
                            Mnemonic::SLA => self.sla(r),
                            Mnemonic::SRA => self.sra(r),
                            Mnemonic::SRL => self.srl(r),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::BIT |
            Mnemonic::SET |
            Mnemonic::RES => {
                if let Some(operands) = instruction.operands() {
                    if let (
                        Operand::Value(ValueType::Constant(value)),
                        Operand::Reference(r)
                    ) = (operands[0], operands[1]) {
                        let value = value as u8;
                        match mnemonic {
                            Mnemonic::BIT => self.bit(value, r),
                            Mnemonic::SET => self.set(value, r),
                            Mnemonic::RES => self.res(value, r),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            },
            Mnemonic::JP |
            Mnemonic::JR |
            Mnemonic::CALL => {
                if let Some(operands) = instruction.operands() {
                    if operands.len() == 1 {
                        if let Operand::Value(value) = operands[0] {
                            let address = self.operand_value(value);
                            match mnemonic {
                                Mnemonic::JP => {
                                    self.jp(address);
                                },
                                Mnemonic::JR => {
                                    self.jr(address as i8);
                                },
                                Mnemonic::CALL => {
                                    self.call(address);
                                },
                                _ => {}
                            }
                        } else {
                            return Err("Wrong argument");
                        }
                    } else if operands.len() == 2 {
                        if let (Operand::Condition(condition), Operand::Value(value)) = (operands[0], operands[1]) {
                            if self.operand_condition(condition) {
                                let address = self.operand_value(value);
                                match mnemonic {
                                    Mnemonic::JP => {
                                        self.jp(address);
                                    },
                                    Mnemonic::JR => {
                                        self.jr(address as i8);
                                    },
                                    Mnemonic::CALL => {
                                        self.call(address);
                                    },
                                    _ => {}
                                }
                            }
                        } else {
                            return Err("Wrong arguments");
                        }
                    }
                } else {
                    return Err("Requires arguments");
                }
            },
            Mnemonic::RST => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(ValueType::Constant(v)) = operands[0] {
                        self.rst(v);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires argument");
                }
            },
            Mnemonic::RET => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Condition(condition) = operands[0] {
                        if self.operand_condition(condition) {
                            self.ret();
                        }
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    self.ret();
                }
            },
            Mnemonic::RETI => self.reti(),
            Mnemonic::CB => self.cb(),

            _ => { return Err("Invalid mnemonic"); }
        };
        Ok(())
    }

    fn operand_value(&mut self, value: ValueType) -> u16 {
        match value {
            ValueType::Constant(value) => value,
            ValueType::Register(reg) => self.reg(reg),
            ValueType::Immediate => self.immediate() as u16,
            ValueType::Immediate16 => self.immediate16(),
            ValueType::Address(address) => {
                let address = match address {
                    AddressType::Immediate => self.immediate16(),
                    AddressType::IncImmediate =>
                        self.immediate16() + 0xFF00,
                    AddressType::Register(reg) => self.reg(reg),
                    AddressType::IncRegister(reg) => self.reg(reg) + 0xFF00
                };
                self.address(address) as u16
            }
        }
    }

    fn operand_address(&mut self, address: AddressType) -> u16 {
        match address {
            AddressType::Register(reg) => self.reg(reg),
            AddressType::IncRegister(reg) => self.reg(reg) + 0xFF00,
            AddressType::Immediate => self.immediate16(),
            AddressType::IncImmediate =>
                self.immediate16() + 0xFF00
        }
    }

    fn operand_condition(&self, condition: (Flag, bool)) -> bool {
        let (flag, value) = condition;
        return self.flag(flag) == value;
    }

    fn reference(&mut self, reference: Reference) -> u16 {
        match reference {
            Reference::Register(register) => self.reg(register),
            Reference::Address(address) => self.operand_address(address)
        }
    }

    fn set_reference(&mut self, reference: Reference, value: u16) {
        match reference {
            Reference::Register(register) => {
                self.set_reg(register, value);
            }
            Reference::Address(address) => {
                let address = self.operand_address(address);
                self.set_address(address, value as u8);
                if value > 0xFF {
                    self.set_address(address + 1, (value >> 8) as u8);
                }
            }
        };
    }

    fn ld(&mut self, reference: Reference, value: u16) {
        self.set_reference(reference, value);
    }

    fn ldd(&mut self, r1: Reference, r2: Reference) {
        let value = self.reference(r2);
        self.ld(r1, value);
        if let Reference::Address(_) = r1 {
            self.dec(r1);
        } else {
            self.dec(r2);
        }
    }

    fn ldi(&mut self, r1: Reference, r2: Reference) {
        let value = self.reference(r2);
        self.ld(r1, value);
        if let Reference::Address(_) = r1 {
            self.inc(r1);
        } else {
            self.inc(r2);
        }
    }

    // writes SP + n to HL
    fn ldhl(&mut self) {
        let n = self.immediate() as i32;
        let value = (self.reg(RegisterType::SP) as i32 + n) as u32;
        self.set_flag(Flag::HalfCarry, value > 0xFFF);
        self.set_flag(Flag::Carry, value > 0xFFFF);
        self.set_flag(Flag::Zero, false);
        self.set_flag(Flag::AddSub, false);
        self.set_reg(RegisterType::HL, value as u16);
    }

    fn push(&mut self, value: u16) {
        self.push_stack(value);
    }

    fn pop(&mut self, register: RegisterType) {
        let value = self.pop_stack();
        self.set_reg(register, value);
    }

    fn add(&mut self, register: RegisterType, value: u8) {
        let reg_value = self.reg(register);
        let result = reg_value + value as u16;
        self.set_reg(register, result);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::HalfCarry, result > 0xF);
        self.set_flag(Flag::Carry, result > 0xFF);
    }

    fn add16(&mut self, register: RegisterType, value: u16) {
        let reg_value = self.reg(register) as u32;
        let result = reg_value + value as u32;
        self.set_reg(register, result as u16);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, result > 0xFFF);
        self.set_flag(Flag::Carry, result > 0xFFFF);

        // somehow, the zero is only touched if SP is the first argument
        if let RegisterType::SP = register {
            self.set_flag(Flag::Zero, false);
        }
    }

    fn adc(&mut self, value: u8) {
        let carry = self.flag(Flag::Carry) as u8;
        self.add(RegisterType::A, value + carry);
    }

    fn sub(&mut self, value: u8) {
        let reg_value = self.reg(RegisterType::A);
        let result = reg_value - value as u16;

        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(
            Flag::HalfCarry,
            reg_value & 0xF < value as u16 & 0xF
        );
        self.set_flag(Flag::Carry, reg_value < value as u16);
    }

    fn sbc(&mut self, value: u8) {
        let carry = self.flag(Flag::Carry);
        self.sub(value + carry as u8);
    }

    fn and(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = (a & value) as u16;
        self.set_reg(RegisterType::A, result);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, true);
        self.set_flag(Flag::Carry, false);
    }

    fn or(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = (a | value) as u16;
        self.set_reg(RegisterType::A, result);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, false);
    }

    fn xor(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = (a ^ value) as u16;
        self.set_reg(RegisterType::A, result);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, false);
    }

    fn cp(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = (a - value) as u16;
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, true);
        self.set_flag(
            Flag::HalfCarry,
            result & 0xF < value as u16 & 0xF
        );
        self.set_flag(Flag::Carry, result < value as u16);
    }

    fn inc(&mut self, reference: Reference) {
        let value = self.reference(reference);
        self.set_reference(reference, value + 1);
    }

    fn dec(&mut self, reference: Reference) {
        let value = self.reference(reference);
        self.set_reference(reference, value - 1);
    }

    fn daa(&mut self) {
        let mut a = self.reg(RegisterType::A);
        if self.flag(Flag::AddSub) {
            if self.flag(Flag::HalfCarry) {
                a += 0xFA;
            }
            if self.flag(Flag::Carry) {
                a += 0xA0;
            }
        } else {
            if a & 0xF > 9 || self.flag(Flag::HalfCarry) {
                a += 0x06;
            }
            // TODO: might be stupid
            if a > 0x90 {
                a += 0x60;
                self.set_flag(Flag::Carry, true);
            } else {
                self.set_flag(Flag::Carry, false);
            }
        }
        self.set_reg(RegisterType::A, a);
        self.set_flag(Flag::Zero, a == 0);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn cpl(&mut self) {
        let a = self.reg(RegisterType::A);
        self.set_reg(RegisterType::A, !a);
        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::HalfCarry, true);
    }

    fn ccf(&mut self) {
        let value = self.flag(Flag::Carry);
        self.set_flag(Flag::Carry, !value);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn scf(&mut self) {
        self.set_flag(Flag::Carry, true);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn halt(&mut self) {
        unimplemented!()
    }

    fn stop(&mut self) {
        unimplemented!()
    }

    fn di(&mut self) {
        unimplemented!()
    }

    fn ei(&mut self) {
        unimplemented!()
    }

    fn cb(&mut self) {
        self.execute_next(Prefix::CB);
    }

    fn rlc(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn rl(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn rrc(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn rr(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn sla(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn sra(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn srl(&mut self, reference: Reference) {
        unimplemented!();
    }

    fn bit(&mut self, bit: u8, reference: Reference) {
        let value = self.reference(reference) as u8;
        self.set_flag(Flag::Zero, bits::get_bit(value, bit));
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, true);
    }

    fn set(&mut self, bit: u8, reference: Reference) {
        let value = self.reference(reference) as u8;
        self.set_reference(reference, bits::set_bit(value, bit, true) as u16);
    }

    fn res(&mut self, bit: u8, reference: Reference) {
        let value = self.reference(reference) as u8;
        self.set_reference(reference, bits::set_bit(value, bit, false) as u16);
    }

    fn jp(&mut self, address: u16) {
        self.set_reg(RegisterType::PC, address);
    }

    fn jr(&mut self, inc: i8) {
        let pc = self.reg(RegisterType::PC) as i32;
        let result = pc - inc as i32;
        self.set_reg(RegisterType::PC, result as u16);
    }

    fn call(&mut self, address: u16) {
        let pc = self.reg(RegisterType::PC);
        self.push_stack(pc);
        self.jp(address);
    }

    fn rst(&mut self, address: u16) {
        let pc = self.reg(RegisterType::PC);
        self.push_stack(pc);
        self.jp(address);
    }

    fn ret(&mut self) {
        let address = self.pop_stack();
        self.jp(address);
    }

    fn reti(&mut self) {
        self.ret();
        self.ei();
    }
}