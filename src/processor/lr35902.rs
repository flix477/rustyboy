use crate::bus::Bus;
use crate::processor::flag_register::Flag;
use crate::processor::instruction::Prefix;
use crate::processor::instruction::Reference;
use crate::processor::instruction::{AddressType, Operand, ValueType};
use crate::processor::instruction::{InstructionInfo, Mnemonic};
use crate::processor::registers::RegisterType;
use crate::util::bits;

pub trait LR35902 {
    fn immediate<H: Bus>(&mut self, bus: &H) -> u8;
    fn immediate16<H: Bus>(&mut self, bus: &H) -> u16;
    fn reg(&self, register: RegisterType) -> u16;
    fn set_reg(&mut self, register: RegisterType, value: u16);
    fn address<H: Bus>(&self, bus: &H, address: u16) -> u8;
    fn set_address<H: Bus>(&self, bus: &mut H, address: u16, value: u8);
    fn flag(&self, flag: Flag) -> bool;
    fn set_flag(&mut self, flag: Flag, value: bool);
    fn push_stack<H: Bus>(&mut self, bus: &mut H, value: u16);
    fn pop_stack<H: Bus>(&mut self, bus: &mut H) -> u16;
    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) -> u8;
    fn halt(&mut self);
    fn stop(&mut self);

    fn execute<H: Bus>(&mut self, bus: &mut H, instruction: InstructionInfo) -> Result<(), &str> {
        let mnemonic = *instruction.mnemonic();
        match mnemonic {
            Mnemonic::LD => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Reference(r), Operand::Value(v)) = (operands[0], operands[1]) {
                        let value = self.operand_value(bus, v);
                        self.ld(bus, r, value);
                    } else {
                        return Err("Wrong arguments");
                    }
                } else {
                    return Err("Requires arguments");
                }
            }
            Mnemonic::LDD | Mnemonic::LDI => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Reference(r), Operand::Value(v)) = (operands[0], operands[1]) {
                        match mnemonic {
                            Mnemonic::LDD => self.ldd(bus, r, v),
                            Mnemonic::LDI => self.ldi(bus, r, v),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong arguments");
                    }
                } else {
                    return Err("Requires arguments");
                }
            }
            Mnemonic::LDHL => self.ldhl(bus),
            Mnemonic::PUSH => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(value) = operands[0] {
                        let value = self.operand_value(bus, value);
                        self.push(bus, value);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::POP => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(Reference::Register(r)) = operands[0] {
                        self.pop(bus, r);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::ADD => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Reference(Reference::Register(r)), Operand::Value(v)) =
                        (operands[0], operands[1])
                    {
                        let value = self.operand_value(bus, v);
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
            Mnemonic::ADC
            | Mnemonic::SUB
            | Mnemonic::SBC
            | Mnemonic::AND
            | Mnemonic::OR
            | Mnemonic::XOR => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(v) = operands[0] {
                        let value = self.operand_value(bus, v) as u8;
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
            }
            Mnemonic::CP => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(value) = operands[0] {
                        let value = self.operand_value(bus, value);
                        self.cp(value as u8);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::INC | Mnemonic::DEC => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(reference) = operands[0] {
                        if let Mnemonic::INC = mnemonic {
                            self.inc(bus, reference);
                        } else {
                            self.dec(bus, reference);
                        }
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::DAA => self.daa(),
            Mnemonic::CPL => self.cpl(),
            Mnemonic::CCF => self.ccf(),
            Mnemonic::SCF => self.scf(),
            Mnemonic::NOP => {}
            Mnemonic::HALT => self.halt(),
            Mnemonic::STOP => self.stop(),
            Mnemonic::DI => self.di(bus),
            Mnemonic::EI => self.ei(bus),
            Mnemonic::RLC | Mnemonic::RL | Mnemonic::RRC | Mnemonic::RR | Mnemonic::SWAP => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(r) = operands[0] {
                        match mnemonic {
                            Mnemonic::RLC => self.rlc(bus, r),
                            Mnemonic::RL => self.rl(bus, r),
                            Mnemonic::RRC => self.rrc(bus, r),
                            Mnemonic::RR => self.rr(bus, r),
                            Mnemonic::SWAP => self.swap(bus, r),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::SLA | Mnemonic::SRA | Mnemonic::SRL => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Reference(r) = operands[0] {
                        match mnemonic {
                            Mnemonic::SLA => self.sla(bus, r),
                            Mnemonic::SRA => self.sra(bus, r),
                            Mnemonic::SRL => self.srl(bus, r),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::BIT | Mnemonic::SET | Mnemonic::RES => {
                if let Some(operands) = instruction.operands() {
                    if let (Operand::Value(ValueType::Constant(value)), Operand::Reference(r)) =
                        (operands[0], operands[1])
                    {
                        let value = value as u8;
                        match mnemonic {
                            Mnemonic::BIT => self.bit(bus, value, r),
                            Mnemonic::SET => self.set(bus, value, r),
                            Mnemonic::RES => self.res(bus, value, r),
                            _ => {}
                        };
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires an argument");
                }
            }
            Mnemonic::JP | Mnemonic::JR | Mnemonic::CALL => {
                if let Some(operands) = instruction.operands() {
                    if operands.len() == 1 {
                        if let Operand::Value(value) = operands[0] {
                            let address = self.operand_value(bus, value);
                            match mnemonic {
                                Mnemonic::JP => {
                                    self.jp(address);
                                }
                                Mnemonic::JR => {
                                    self.jr(address as i8);
                                }
                                Mnemonic::CALL => {
                                    self.call(bus, address);
                                }
                                _ => {}
                            }
                        } else {
                            return Err("Wrong argument");
                        }
                    } else if operands.len() == 2 {
                        if let (Operand::Condition(condition), Operand::Value(value)) =
                            (operands[0], operands[1])
                        {
                            let address = self.operand_value(bus, value);
                            if self.operand_condition(condition) {
                                match mnemonic {
                                    Mnemonic::JP => {
                                        self.jp(address);
                                    }
                                    Mnemonic::JR => {
                                        self.jr(address as i8);
                                    }
                                    Mnemonic::CALL => {
                                        self.call(bus, address);
                                    }
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
            }
            Mnemonic::RST => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Value(ValueType::Constant(v)) = operands[0] {
                        self.rst(bus, v);
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    return Err("Requires argument");
                }
            }
            Mnemonic::RET => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Condition(condition) = operands[0] {
                        if self.operand_condition(condition) {
                            self.ret(bus);
                        }
                    } else {
                        return Err("Wrong argument");
                    }
                } else {
                    self.ret(bus);
                }
            }
            Mnemonic::RETI => self.reti(bus),
            Mnemonic::CB => self.cb(bus),
        };
        Ok(())
    }

    fn operand_value<H: Bus>(&mut self, bus: &H, value: ValueType) -> u16 {
        match value {
            ValueType::Constant(value) => value,
            ValueType::Register(reg) => self.reg(reg),
            ValueType::Immediate => self.immediate(bus) as u16,
            ValueType::Immediate16 => self.immediate16(bus),
            ValueType::Address(address) => {
                let address = match address {
                    AddressType::Immediate => self.immediate16(bus),
                    AddressType::IncImmediate => (self.immediate(bus) as u16).wrapping_add(0xFF00),
                    AddressType::Register(reg) => self.reg(reg),
                    AddressType::IncRegister(reg) => self.reg(reg) + 0xFF00,
                };
                self.address(bus, address) as u16
            }
        }
    }

    fn operand_address<H: Bus>(&mut self, bus: &mut H, address: AddressType) -> u16 {
        match address {
            AddressType::Register(reg) => self.reg(reg),
            AddressType::IncRegister(reg) => self.reg(reg) + 0xFF00,
            AddressType::Immediate => self.immediate16(bus),
            AddressType::IncImmediate => (self.immediate(bus) as u16).wrapping_add(0xFF00),
        }
    }

    fn operand_condition(&self, condition: (Flag, bool)) -> bool {
        let (flag, value) = condition;
        return self.flag(flag) == value;
    }

    fn reference<H: Bus>(&mut self, bus: &mut H, reference: Reference) -> u16 {
        match reference {
            Reference::Register(register) => self.reg(register),
            Reference::Address(address) => self.operand_address(bus, address),
        }
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
        let n = self.immediate(bus) as i32;
        let value = (self.reg(RegisterType::SP) as i32 + n) as u32;
        self.set_flag(Flag::HalfCarry, value > 0xFFF);
        self.set_flag(Flag::Carry, value > 0xFFFF);
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
        self.add(RegisterType::A, value.wrapping_add(carry));
    }

    fn sub(&mut self, value: u8) {
        let value = value as u16;
        let reg_value = self.reg(RegisterType::A);
        let result = reg_value.wrapping_sub(value);

        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::HalfCarry, reg_value & 0xF < value & 0xF);
        self.set_flag(Flag::Carry, reg_value < value);
    }

    fn sbc(&mut self, value: u8) {
        let carry = self.flag(Flag::Carry);
        self.sub(value.wrapping_add(carry as u8));
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
        let result = a.wrapping_sub(value) as u16;
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::HalfCarry, result & 0xF < value as u16 & 0xF);
        self.set_flag(Flag::Carry, result < value as u16);
    }

    fn inc<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        let result = value as u32 + 1;

        let safe_result = value.wrapping_add(1);
        self.set_reference(bus, reference, safe_result);
        let safe_result = self.reference(bus, reference);

        if let Reference::Register(reg) = reference {
            if reg.is16bit() {
                return;
            }
        }

        self.set_flag(Flag::Zero, safe_result == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, result > 0xF);
        self.set_flag(Flag::Carry, result > 0xFF);
    }

    fn dec<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        let result = value.wrapping_sub(1);
        self.set_reference(bus, reference, result);
        let result = self.reference(bus, reference);

        if let Reference::Register(reg) = reference {
            if reg.is16bit() {
                return;
            }
        }

        self.set_flag(Flag::AddSub, true);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::HalfCarry, value & 0xF < 1);
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

    fn di<H: Bus>(&mut self, bus: &mut H) {
        bus.toggle_interrupts(false);
    }

    fn ei<H: Bus>(&mut self, bus: &mut H) {
        bus.toggle_interrupts(true);
    }

    fn cb<H: Bus>(&mut self, bus: &mut H) {
        self.execute_next(bus, Prefix::CB);
    }

    fn rlc<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 7));

        value.rotate_left(1);
        self.set_reference(bus, reference, value);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn rl<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        let old_flag = self.flag(Flag::Carry) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 7));

        let value = (value << 1) | old_flag;
        self.set_reference(bus, reference, value as u16);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn rrc<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 0));

        value.rotate_right(1);
        self.set_reference(bus, reference, value);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn rr<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        let old_flag = self.flag(Flag::Carry) as u8;
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 0));

        let value = (value >> 1) | (old_flag << 7);
        self.set_reference(bus, reference, value as u16);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
    }

    fn swap<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference) as u8;

        let value = value.rotate_left(4);
        self.set_reference(bus, reference, value as u16);

        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, false);
    }

    fn sla<H: Bus>(&mut self, bus: &mut H, reference: Reference) {
        let value = self.reference(bus, reference);
        self.set_flag(Flag::Carry, bits::get_bit(value as u8, 7));

        let value = value << 1;
        self.set_reference(bus, reference, value);

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
        self.set_flag(Flag::Zero, bits::get_bit(value, bit));
        self.set_flag(Flag::AddSub, false);
        self.set_flag(Flag::HalfCarry, true);
    }

    fn set<H: Bus>(&mut self, bus: &mut H, bit: u8, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        self.set_reference(bus, reference, bits::set_bit(value, bit, true) as u16);
    }

    fn res<H: Bus>(&mut self, bus: &mut H, bit: u8, reference: Reference) {
        let value = self.reference(bus, reference) as u8;
        self.set_reference(bus, reference, bits::set_bit(value, bit, false) as u16);
    }

    fn jp(&mut self, address: u16) {
        self.set_reg(RegisterType::PC, address);
    }

    fn jr(&mut self, inc: i8) {
        let pc = self.reg(RegisterType::PC) as i32;
        let result = pc + inc as i32;
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
        self.ei(bus);
    }
}
