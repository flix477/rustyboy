use crate::bus::Bus;
use crate::processor::instruction::{InstructionInfo, Mnemonic, Operand, Prefix, Reference, ValueType, Condition};
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::flag_register::{
    carry_add, half_carry_add, half_carry_add16, half_carry_sub, Flag,
};
use crate::processor::registers::RegisterType;
use crate::util::bits;

pub struct InstructionInfo2 {
    mnemonic: Mnemonic2,
    cycle_count: u16
}

pub enum Mnemonic2 {
    LD(Reference, ValueType),
    LDD(Reference, ValueType),
    LDI(Reference, ValueType),
    LDHL,
    PUSH(ValueType),
    POP(RegisterType),
    ADD(RegisterType, ValueType),
    ADC(ValueType),
    SUB(ValueType),
    SBC(ValueType),
    AND(ValueType),
    OR(ValueType),
    XOR(ValueType),
    CP(ValueType),
    INC(Reference),
    DEC(Reference),
    DAA,
    CPL,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    RLC(Reference),
    RL(Reference),
    RRC(Reference),
    RR(Reference),
    SWAP(Reference),
    RLCA,
    RLA,
    RRCA,
    RRA,
    SLA(Reference),
    SRA(Reference),
    SRL(Reference),
    BIT(u16, Reference),
    SET(u16, Reference),
    RES(u16, Reference),
    JP(Option<Condition>, ValueType),
    JR(Option<Condition>, ValueType),
    CALL(Option<Condition>, ValueType),
    RST(u16),
    RET(Option<Condition>),
    RETI,
    CB
}

pub trait LR35902: OperandParser {
    fn set_reg(&mut self, register: RegisterType, value: u16);
    fn set_address<H: Bus>(&self, bus: &mut H, address: u16, value: u8);
    fn set_flag(&mut self, flag: Flag, value: bool);
    fn push_stack<H: Bus>(&mut self, bus: &mut H, value: u16);
    fn pop_stack<H: Bus>(&mut self, bus: &mut H) -> u16;
    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) -> u8;
    fn halt<H: Bus>(&mut self, bus: &H);
    fn stop(&mut self);

    fn execute<H: Bus>(&mut self, bus: &mut H, instruction: InstructionInfo2) -> Result<(), &str> {
        let InstructionInfo2 {mnemonic, ..} = instruction;
        match mnemonic {
            Mnemonic2::LD => {
                if let [Some(Operand::Reference(r)), Some(Operand::Value(v))] = operands
                {
                    let value = self.operand_value(bus, v);
                    self.ld(bus, r, value);
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::LDD => {
                if let [Some(Operand::Reference(r)), Some(Operand::Value(v))] = operands {
                    self.ldd(bus, r, v);
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::LDI => {
                if let [Some(Operand::Reference(r)), Some(Operand::Value(v))] = operands {
                    self.ldi(bus, r, v);
                } else {
                    return Err("Wrong arguments");
                }
            }
            Mnemonic2::LDHL => self.ldhl(bus),
            Mnemonic2::PUSH => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value);
                    self.push(bus, value);
                } else {
                    return Err("Wrong argument");
                }
            }
            Mnemonic2::POP => {
                if let [Some(Operand::Reference(Reference::Register(r))), _] = operands {
                    self.pop(bus, r);
                } else {
                    return Err("Wrong argument");
                }
            }
            Mnemonic2::ADD => {
                if let [Some(Operand::Reference(Reference::Register(r))), Some(Operand::Value(v))] =
                    operands
                {
                    let value = self.operand_value(bus, v);
                    if r.is16bit() {
                        if r == RegisterType::SP {
                            self.add_sp(value as i8);
                        } else {
                            self.add16(r, value);
                        }
                    } else {
                        self.add(r, value as u8);
                    }
                } else {
                    return Err("Wrong arguments");
                }
            }
            Mnemonic2::ADC => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value) as u8;
                    self.adc(value)
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::SUB => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value) as u8;
                    self.sub(value)
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::SBC => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value) as u8;
                    self.sbc(value)
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::AND => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value) as u8;
                    self.and(value)
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::OR => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value) as u8;
                    self.or(value)
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::XOR => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value) as u8;
                    self.xor(value)
                } else {
                    return Err("Wrong arguments");
                }
            },
            Mnemonic2::CP => {
                if let [Some(Operand::Value(value)), _] = operands {
                    let value = self.operand_value(bus, value);
                    self.cp(value as u8);
                } else {
                    return Err("Wrong argument");
                }
            }
            Mnemonic2::INC | Mnemonic2::DEC => {
                if let [Some(Operand::Reference(reference)), _] = operands {
                    if let Mnemonic2::INC = mnemonic {
                        self.inc(bus, reference);
                    } else {
                        self.dec(bus, reference);
                    }
                } else {
                    return Err("Wrong argument");
                }
            }
            Mnemonic2::DAA => self.daa(),
            Mnemonic2::CPL => self.cpl(),
            Mnemonic2::CCF => self.ccf(),
            Mnemonic2::SCF => self.scf(),
            Mnemonic2::NOP => {}
            Mnemonic2::HALT => self.halt(bus),
            Mnemonic2::STOP => self.stop(),
            Mnemonic2::DI => self.di(bus),
            Mnemonic2::EI => self.ei(),
            Mnemonic2::RLC => {
                if let [Some(Operand::Reference(reference)), _] = operands {
                    self.rlc(bus, reference);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::RL => {
                if let [Some(Operand::Reference(reference)), _] = operands {
                    self.rl(bus, reference);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::RRC => {
                if let [Some(Operand::Reference(reference)), _] = operands {
                    self.rrc(bus, reference);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::RR => {
                if let [Some(Operand::Reference(reference)), _] = operands {
                    self.rr(bus, reference);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::SWAP => {
                if let [Some(Operand::Reference(reference)), _] = operands {
                    self.swap(bus, reference);
                } else {
                    return Err("Wrong argument");
                }
            }
            Mnemonic2::RLCA => self.rlca(bus),
            Mnemonic2::RLA => self.rla(bus),
            Mnemonic2::RRCA => self.rrca(bus),
            Mnemonic2::RRA => self.rra(bus),
            Mnemonic2::SLA => {
                if let [Some(Operand::Reference(r)), _] = operands {
                    self.sla(bus, r);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::SRA => {
                if let [Some(Operand::Reference(r)), _] = operands {
                    self.sra(bus, r);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::SRL => {
                if let [Some(Operand::Reference(r)), _] = operands {
                    self.srl(bus, r);
                } else {
                    return Err("Wrong argument");
                }
            }
            Mnemonic2::BIT => {
                if let [Some(Operand::Value(ValueType::Constant(value))), Some(Operand::Reference(r))] = operands {
                    self.bit(bus, value as u8, r);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::SET => {
                if let [Some(Operand::Value(ValueType::Constant(value))), Some(Operand::Reference(r))] = operands {
                    self.set(bus, value as u8, r);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::RES => {
                if let [Some(Operand::Value(ValueType::Constant(value))), Some(Operand::Reference(r))] = operands {
                    self.res(bus, value as u8, r);
                } else {
                    return Err("Wrong argument");
                }
            },
            Mnemonic2::JP => {
                match operands {
                    [Some(Operand::Condition(condition)), Some(Operand::Value(value))] => {
                        let address = self.operand_value(bus, value);
                        if self.operand_condition(condition) {
                            self.jp(address);
                        }
                    },
                    [Some(Operand::Value(value)), _] => {
                        let address = self.operand_value(bus, value);
                        self.jp(address);
                    }
                    _ => {
                        return Err("Requires arguments");
                    }
                }
            },
            Mnemonic2::JR => {
                match operands {
                    [Some(Operand::Condition(condition)), Some(Operand::Value(value))] => {
                        let address = self.operand_value(bus, value);
                        if self.operand_condition(condition) {
                            self.jr(address as i8);
                        }
                    },
                    [Some(Operand::Value(value)), _] => {
                        let address = self.operand_value(bus, value);
                        self.jr(address as i8);
                    }
                    _ => {
                        return Err("Requires arguments");
                    }
                }
            },
            Mnemonic2::CALL => {
                match operands {
                    [Some(Operand::Condition(condition)), Some(Operand::Value(value))] => {
                        let address = self.operand_value(bus, value);
                        if self.operand_condition(condition) {
                            self.jr(address as i8);
                        }
                    },
                    [Some(Operand::Value(value)), _] => {
                        let address = self.operand_value(bus, value);
                        self.jr(address as i8);
                    }
                    _ => {
                        return Err("Requires arguments");
                    }
                }
            },
            Mnemonic2::RST => {
                if let [Some(Operand::Value(ValueType::Constant(v))), _] = operands {
                    self.rst(bus, v);
                } else {
                    return Err("Requires argument");
                }
            }
            Mnemonic2::RET => {
                if let [Some(Operand::Condition(condition)), _] = operands {
                    if self.operand_condition(condition) {
                        self.ret(bus);
                    }
                } else {
                    self.ret(bus);
                }
            }
            Mnemonic2::RETI => self.reti(bus),
            Mnemonic2::CB => self.cb(bus),
        };
        Ok(())
    }

    fn set_reference<H: Bus>(&mut self, bus: &mut H, reference: Reference, value: u16) {
        match reference {
            Reference::Register(register) => {
                self.set_reg(register, value);
            }
            Reference::Address(address) => {
                let address = self.operand_address(bus, address);
                self.set_address(bus, address, value as u8);
                if value > 0xFF {
                    self.set_address(bus, address + 1, (value >> 8) as u8);
                }
            }
        };
    }

    fn ld<H: Bus>(&mut self, bus: &mut H, reference: Reference, value: u16) {
        self.set_reference(bus, reference, value);
    }

    fn ldd<H: Bus>(&mut self, bus: &mut H, reg: Reference, value: ValueType) {
        let value = self.operand_value(bus, value);
        self.ld(bus, reg, value);
        self.dec(bus, Reference::Register(RegisterType::HL));
    }

    fn ldi<H: Bus>(&mut self, bus: &mut H, reg: Reference, value: ValueType) {
        let value = self.operand_value(bus, value);
        self.ld(bus, reg, value);
        self.inc(bus, Reference::Register(RegisterType::HL));
    }

    // writes SP + n to HL
    fn ldhl<H: Bus>(&mut self, bus: &H) {
        let n = self.immediate(bus) as i8 as u16;
        let sp = self.reg(RegisterType::SP);
        let value = sp.wrapping_add(n);
        self.set_flag(Flag::HalfCarry, half_carry_add(sp as u8, n as u8));
        self.set_flag(Flag::Carry, carry_add(sp as u8, n as u8));
        self.set_flag(Flag::Zero, false);
        self.set_flag(Flag::AddSub, false);
        self.set_reg(RegisterType::HL, value as u16);
    }

    fn push<H: Bus>(&mut self, bus: &mut H, value: u16) {
        self.push_stack(bus, value);
    }

    fn pop<H: Bus>(&mut self, bus: &mut H, register: RegisterType) {
        let value = self.pop_stack(bus);
        self.set_reg(register, value);
    }

    fn add(&mut self, register: RegisterType, value: u8) {
        let reg_value = self.reg(register) as u8;
        let (result, carry) = reg_value.overflowing_add(value);
        self.set_reg(register, result.into());
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::HalfCarry, half_carry_add(reg_value, value));
        self.set_flag(Flag::Carry, carry);
    }

    fn add16(&mut self, register: RegisterType, value: u16) {
        let reg_value = self.reg(register);
        let (result, carry) = reg_value.overflowing_add(value);
        self.set_reg(register, result);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, half_carry_add16(reg_value, value));
        self.set_flag(Flag::Carry, carry);
    }

    fn add_sp(&mut self, value: i8) {
        let reg_value = self.reg(RegisterType::SP);
        let result = reg_value.wrapping_add(value as u16);
        self.set_reg(RegisterType::SP, result);

        self.set_flag(Flag::Zero, false);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(
            Flag::HalfCarry,
            half_carry_add(reg_value as u8, value as u8),
        );
        self.set_flag(Flag::Carry, carry_add(reg_value as u8, value as u8));
    }

    fn adc(&mut self, value: u8) {
        let carry = self.flag(Flag::Carry) as u8;
        let a = self.reg(RegisterType::A) as u8;
        let (add_value, overflow) = value.overflowing_add(carry);
        let (result, result_overflow) = a.overflowing_add(add_value);
        self.set_reg(RegisterType::A, result.into());
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(
            Flag::HalfCarry,
            half_carry_add(value, carry) || half_carry_add(a, add_value),
        );
        self.set_flag(Flag::Carry, overflow || result_overflow);
    }

    fn base_sub(&mut self, value: u8) -> u8 {
        let reg_value = self.reg(RegisterType::A) as u8;
        let result = reg_value.wrapping_sub(value);

        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::HalfCarry, half_carry_sub(reg_value, value));
        self.set_flag(Flag::Carry, reg_value < value);
        result
    }

    fn sub(&mut self, value: u8) {
        let result = self.base_sub(value);
        self.set_reg(RegisterType::A, u16::from(result));
    }

    fn sbc(&mut self, value: u8) {
        let carry = self.flag(Flag::Carry) as u8;
        let reg_value = self.reg(RegisterType::A) as u8;
        let (sub_value, overflow) = value.overflowing_add(carry);
        let result = reg_value.wrapping_sub(sub_value);
        self.set_reg(RegisterType::A, u16::from(result));

        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(
            Flag::HalfCarry,
            reg_value & 0xF < (value & 0xF).wrapping_add(carry),
        );
        self.set_flag(Flag::Carry, overflow || reg_value < sub_value);
    }

    fn and(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = a & value;
        self.set_reg(RegisterType::A, u16::from(result));
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, true);
        self.set_flag(Flag::Carry, false);
    }

    fn or(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = u16::from(a | value);
        self.set_reg(RegisterType::A, result);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, false);
    }

    fn xor(&mut self, value: u8) {
        let a = self.reg(RegisterType::A) as u8;
        let result = u16::from(a ^ value);
        self.set_reg(RegisterType::A, result);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, false);
    }

    fn cp(&mut self, value: u8) {
        self.base_sub(value);
    }

    fn inc<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        if reference.is16bit() {
            self.inc16(bus, reference)
        } else {
            self.inc8(bus, reference)
        }
    }

    fn inc8<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        let result = value.wrapping_add(1);
        self.set_reference(bus, reference, u16::from(result));

        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, half_carry_add(value as u8, 1));
    }

    fn inc16<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u16;
        self.set_reference(bus, reference, value.wrapping_add(1));
    }

    fn dec<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        if reference.is16bit() {
            self.dec16(bus, reference)
        } else {
            self.dec8(bus, reference)
        }
    }

    fn dec8<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        let result = value.wrapping_sub(1);
        self.set_reference(bus, reference, u16::from(result));

        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::HalfCarry, value & 0xF < 1);
    }

    fn dec16<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u16;
        self.set_reference(bus, reference, value.wrapping_sub(1));
    }

    // implementation stolen from https://forums.nesdev.com/viewtopic.php?f=20&t=15944#p196282
    fn daa(&mut self) {
        let mut a = self.reg(RegisterType::A) as u8;
        let flag_n = self.flag(Flag::AddSub);
        let mut u = if self.flag(Flag::HalfCarry) || (!flag_n && (a & 0xf) > 9) {
            6
        } else {
            0
        };
        if self.flag(Flag::Carry) || (!flag_n && a > 0x99) {
            u |= 0x60;
            self.set_flag(Flag::Carry, true);
        }
        a = if flag_n {
            a.wrapping_sub(u)
        } else {
            a.wrapping_add(u)
        };

        self.set_reg(RegisterType::A, u16::from(a));
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

    fn di<H: Bus>(&mut self, bus: &mut H) {
        bus.toggle_interrupts(false);
    }

    fn ei(&mut self);
    fn immediate_ei<H: Bus>(&mut self, bus: &mut H);

    fn cb<H: Bus>(&mut self, bus: &mut H) {
        self.execute_next(bus, Prefix::CB);
    }

    fn base_rlc<H: Bus>(&mut self, bus: &mut H, reference: Reference) -> u8 {
        let value = self.reference(bus, reference) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value, 7));

        let result = value.rotate_left(1);
        self.set_reference(bus, reference, u16::from(result));

        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);

        result
    }

    fn rlc<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let result = self.base_rlc(bus, reference);
        self.set_flag(Flag::Zero, result == 0);
    }

    fn rlca<H: Bus>(&mut self, bus: &mut H) {
        self.base_rlc(bus, Reference::Register(RegisterType::A));
        self.set_flag(Flag::Zero, false);
    }

    fn base_rl<H: Bus>(&mut self, bus: &mut H, reference: Reference) -> u8 {
        let value = self.reference(bus, reference) as u8;
        let old_flag = self.flag(Flag::Carry) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 7));

        let value = (value << 1) | old_flag;
        self.set_reference(bus, reference, u16::from(value));

        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);

        value
    }

    fn rl<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.base_rl(bus, reference);
        self.set_flag(Flag::Zero, value == 0);
    }

    fn rla<H: Bus>(&mut self, bus: &mut H) {
        self.base_rl(bus, Reference::Register(RegisterType::A));
        self.set_flag(Flag::Zero, false);
    }

    fn base_rrc<H: Bus>(&mut self, bus: &mut H, reference: Reference) -> u8 {
        let value = self.reference(bus, reference) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value, 0));

        let result = value.rotate_right(1);
        self.set_reference(bus, reference, u16::from(result));

        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);

        result
    }

    fn rrc<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.base_rrc(bus, reference);
        self.set_flag(Flag::Zero, value == 0);
    }

    fn rrca<H: Bus>(&mut self, bus: &mut H) {
        self.base_rrc(bus, Reference::Register(RegisterType::A));
        self.set_flag(Flag::Zero, false);
    }

    fn base_rr<H: Bus>(&mut self, bus: &mut H, reference: Reference) -> u8 {
        let value = self.reference(bus, reference) as u8;
        let old_flag = self.flag(Flag::Carry) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 0));

        let value = (value >> 1) | (old_flag << 7);
        self.set_reference(bus, reference, u16::from(value));

        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);

        value
    }

    fn rr<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let result = self.base_rr(bus, reference);
        self.set_flag(Flag::Zero, result == 0);
    }

    fn rra<H: Bus>(&mut self, bus: &mut H) {
        self.base_rr(bus, Reference::Register(RegisterType::A));
        self.set_flag(Flag::Zero, false);
    }

    fn swap<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;

        let value = value.rotate_left(4);
        self.set_reference(bus, reference, u16::from(value));

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, false);
    }

    fn sla<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value, 7));

        let value = value << 1;
        self.set_reference(bus, reference, u16::from(value));

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn sra<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        let msb = bits::get_bit(value as u8, 7) as u16;
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 0));

        let value = (value >> 1) | (msb << 7);
        self.set_reference(bus, reference, value);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn srl<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 0));

        let value = value >> 1;
        self.set_reference(bus, reference, value);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn bit<H: Bus>(&mut self, bus: &mut H, bit: u8, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        self.set_flag(Flag::Zero, !bits::get_bit(value, bit));
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, true);
    }

    fn set<H: Bus>(&mut self, bus: &mut H, bit: u8, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        self.set_reference(bus, reference, u16::from(bits::set_bit(value, bit, true)));
    }

    fn res<H: Bus>(&mut self, bus: &mut H, bit: u8, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        self.set_reference(bus, reference, u16::from(bits::set_bit(value, bit, false)));
    }

    fn jp(&mut self, address: u16) {
        self.set_reg(RegisterType::PC, address);
    }

    fn jr(&mut self, inc: i8) {
        let pc = i32::from(self.reg(RegisterType::PC));
        let result = pc + i32::from(inc);
        self.set_reg(RegisterType::PC, result as u16);
    }

    fn call<H: Bus>(&mut self, bus: &mut H, address: u16) {
        let pc = self.reg(RegisterType::PC);
        self.push_stack(bus, pc);
        self.jp(address);
    }

    fn rst<H: Bus>(&mut self, bus: &mut H, address: u16) {
        let pc = self.reg(RegisterType::PC);
        self.push_stack(bus, pc);
        self.jp(address);
    }

    fn ret<H: Bus>(&mut self, bus: &mut H) {
        let address = self.pop_stack(bus);
        self.jp(address);
    }

    fn reti<H: Bus>(&mut self, bus: &mut H) {
        self.ret(bus);
        self.immediate_ei(bus);
    }
}
