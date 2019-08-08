use crate::bus::Bus;
use crate::processor::instruction::{InstructionInfo, Mnemonic, Prefix, Reference, ValueType};
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::flag_register::{
    carry_add, half_carry_add, half_carry_add16, half_carry_sub, Flag,
};
use crate::processor::registers::RegisterType;
use crate::util::bits;

pub trait LR35902: OperandParser {
    fn set_reg(&mut self, register: RegisterType, value: u16);
    fn set_address<H: Bus>(&self, bus: &mut H, address: u16, value: u8);
    fn set_flag(&mut self, flag: Flag, value: bool);
    fn push_stack<H: Bus>(&mut self, bus: &mut H, value: u16);
    fn pop_stack<H: Bus>(&mut self, bus: &mut H) -> u16;
    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) -> u8;
    fn halt<H: Bus>(&mut self, bus: &H);
    fn stop(&mut self);

    fn execute<H: Bus>(&mut self, bus: &mut H, instruction: InstructionInfo) {
        let InstructionInfo {mnemonic, ..} = instruction;
        match mnemonic {
            Mnemonic::LD(reference, value) => {
                let value = self.operand_value(bus, value);
                self.ld(bus, reference, value);
            },
            Mnemonic::LDD(reference, value) => self.ldd(bus, reference, value),
            Mnemonic::LDI(reference, value) => self.ldi(bus, reference, value),
            Mnemonic::LDHL => self.ldhl(bus),
            Mnemonic::PUSH(value) => {
                let value = self.operand_value(bus, value);
                self.push(bus, value);
            }
            Mnemonic::POP(register) => self.pop(bus, register),
            Mnemonic::ADD(register, value) => {
                let value = self.operand_value(bus, value);
                if register.is16bit() {
                    if register == RegisterType::SP {
                        self.add_sp(value as i8);
                    } else {
                        self.add16(register, value);
                    }
                } else {
                    self.add(register, value as u8);
                }
            }
            Mnemonic::ADC(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.adc(value)
            },
            Mnemonic::SUB(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.sub(value)
            },
            Mnemonic::SBC(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.sbc(value)
            },
            Mnemonic::AND(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.and(value)
            },
            Mnemonic::OR(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.or(value)
            },
            Mnemonic::XOR(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.xor(value)
            },
            Mnemonic::CP(value) => {
                let value = self.operand_value(bus, value) as u8;
                self.cp(value)
            }
            Mnemonic::INC(reference) => self.inc(bus, reference),
            Mnemonic::DEC(reference) => self.dec(bus, reference),
            Mnemonic::DAA => self.daa(),
            Mnemonic::CPL => self.cpl(),
            Mnemonic::CCF => self.ccf(),
            Mnemonic::SCF => self.scf(),
            Mnemonic::NOP => {}
            Mnemonic::HALT => self.halt(bus),
            Mnemonic::STOP => self.stop(),
            Mnemonic::DI => self.di(bus),
            Mnemonic::EI => self.ei(),
            Mnemonic::RLC(reference) => self.rlc(bus, reference),
            Mnemonic::RL(reference) => self.rl(bus, reference),
            Mnemonic::RRC(reference) => self.rrc(bus, reference),
            Mnemonic::RR(reference) => self.rr(bus, reference),
            Mnemonic::SWAP(reference) => self.swap(bus, reference),
            Mnemonic::RLCA => self.rlca(bus),
            Mnemonic::RLA => self.rla(bus),
            Mnemonic::RRCA => self.rrca(bus),
            Mnemonic::RRA => self.rra(bus),
            Mnemonic::SLA(reference) => self.sla(bus, reference),
            Mnemonic::SRA(reference) => self.sra(bus, reference),
            Mnemonic::SRL(reference) => self.srl(bus, reference),
            Mnemonic::BIT(value, reference) => self.bit(bus, value as u8, reference),
            Mnemonic::SET(value, reference) => self.set(bus, value as u8, reference),
            Mnemonic::RES(value, reference) => self.res(bus, value as u8, reference),
            Mnemonic::JP(Some(condition), value) => {
                let address = self.operand_value(bus, value);
                if self.operand_condition(condition) {
                    self.jp(address);
                }
            },
            Mnemonic::JP(_, value) => {
                let address = self.operand_value(bus, value);
                self.jp(address);
            },
            Mnemonic::JR(Some(condition), value) => {
                let address = self.operand_value(bus, value);
                if self.operand_condition(condition) {
                    self.jr(address as i8);
                }
            },
            Mnemonic::JR(_, value) => {
                let address = self.operand_value(bus, value);
                self.jr(address as i8);
            },
            Mnemonic::CALL(Some(condition), value) => {
                let address = self.operand_value(bus, value);
                if self.operand_condition(condition) {
                    self.jr(address as i8);
                }
            },
            Mnemonic::CALL(_, value) => {
                let address = self.operand_value(bus, value);
                self.jr(address as i8);
            },
            Mnemonic::RST(value) => self.rst(bus, value),
            Mnemonic::RET(Some(condition)) => {
                if self.operand_condition(condition) {
                    self.ret(bus);
                }
            },
            Mnemonic::RET(_) => self.ret(bus),
            Mnemonic::RETI => self.reti(bus),
            Mnemonic::CB => self.cb(bus),
        };
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
