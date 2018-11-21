use processor::instruction::*;
use processor::instruction::AddressType as Addr;
use processor::registers::RegisterType as Reg;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8) -> Option<InstructionInfo> {
        match opcode {
            // NOP
            0 => Some(Self::nop(opcode)),

            // LD nn,n
            // put value nn into n
            // TODO: diverging docs, nn->n or n->nn?
            0x06 => Some(Self::ld_rn(opcode, Reg::B)),
            0x0E => Some(Self::ld_rn(opcode, Reg::C)),
            0x16 => Some(Self::ld_rn(opcode, Reg::D)),
            0x1E => Some(Self::ld_rn(opcode, Reg::E)),
            0x26 => Some(Self::ld_rn(opcode, Reg::H)),
            0x2E => Some(Self::ld_rn(opcode, Reg::L)),
            0x36 => Some(Self::ld_rn(opcode, Reg::HL)),

            // HALT
            0x76 => Some(Self::nop(opcode)),

            // LD r1,r2
            // put value r2 in r1
            0x40..=0x7F => Self::parse_ld_rr(opcode),

            // LD A,n
            // put value n into A
            0x0A => Some(Self::ld_rr(opcode, Reg::A, Reg::BC)),
            0x1A => Some(Self::ld_rr(opcode, Reg::A, Reg::DE)),
            0x7E => Some(Self::ld_rr(opcode, Reg::A, Reg::HL)),
            0xFA => Some(Self::ld_rn16(opcode, Reg::A)),
            // TODO: 0x3E

            // LD n,A
            0x02 => Some(Self::ld_rr(opcode, Reg::BC, Reg::A)),
            0x12 => Some(Self::ld_rr(opcode, Reg::DE, Reg::A)),
            0xEA => Some(Self::ld_n16r(opcode, Reg::A)),

            // LD A,(C)
            // Put value at address $FF00 + register C into A
            0xF2 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(
                        ValueType::Address(
                            Addr::IncRegister(Reg::C)
                        )
                    )
                ]),
                8
            )),

            // LD (C),A
            // Put A into address $FF00 + register C
            0xE2 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Address(Addr::IncRegister(Reg::C)),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                8
            )),

            // LDD A,(HL)
            0x3A => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDD,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL)))
                ]),
                8
            )),

            // LDD (HL),A
            0x32 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDD,
                Some(vec![
                    Operand::Address(Addr::Register(Reg::HL)),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                8
            )),

            // LDI A,(HL)
            0x2A => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDI,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL)))
                ]),
                8
            )),

            // LDI (HL),A
            0x22 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDI,
                Some(vec![
                    Operand::Address(Addr::Register(Reg::HL)),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                8
            )),

            // LDH (n),A
            0xE0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Address(Addr::IncrementedImmediate),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                12
            )),

            // LDH A,(n)
            0xF0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(
                        ValueType::Address(Addr::IncrementedImmediate)
                    )
                ]),
                12
            )),

            // LD n,nn 16bit
            0x01 => Some(Self::ld_r16n16(opcode, Reg::BC)),
            0x11 => Some(Self::ld_r16n16(opcode, Reg::DE)),
            0x21 => Some(Self::ld_r16n16(opcode, Reg::HL)),
            0x31 => Some(Self::ld_r16n16(opcode, Reg::SP)),

            // LD SP,HL
            0xF9 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(Reg::SP),
                    Operand::Value(ValueType::Register(Reg::HL))
                ]),
                8
            )),

            // LDHL SP,n
            0xF8 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDHL,
                Some(vec![
                    Operand::Value(ValueType::Register(Reg::SP)),
                    Operand::Value(ValueType::Immediate)
                ]),
                12
            )),

            // LD (nn),SP
            0x08 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Address(Addr::Immediate),
                    Operand::Value(ValueType::Register(Reg::SP))
                ]),
                20
            )),

            // PUSH nn
            0xF5 => Some(Self::push(opcode, Reg::AF)),
            0xC5 => Some(Self::push(opcode, Reg::BC)),
            0xD5 => Some(Self::push(opcode, Reg::DE)),
            0xE5 => Some(Self::push(opcode, Reg::HL)),

            // POP nn
            0xF1 => Some(Self::pop(opcode, Reg::AF)),
            0xC1 => Some(Self::pop(opcode, Reg::BC)),
            0xD1 => Some(Self::pop(opcode, Reg::DE)),
            0xE1 => Some(Self::pop(opcode, Reg::HL)),

            // ADD A,n
            // add n to A
            0x87 => Some(Self::add_an(opcode, Reg::A)),
            0x80 => Some(Self::add_an(opcode, Reg::B)),
            0x81 => Some(Self::add_an(opcode, Reg::C)),
            0x82 => Some(Self::add_an(opcode, Reg::D)),
            0x83 => Some(Self::add_an(opcode, Reg::E)),
            0x84 => Some(Self::add_an(opcode, Reg::H)),
            0x85 => Some(Self::add_an(opcode, Reg::L)),
            0x86 => Some(Self::add_an(opcode, Reg::HL)),
            0xC6 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::ADD,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // ADC A,n
            // Add n + Carry flag to A
            0x8F => Some(Self::adc_an(opcode, Reg::A)),
            0x89 => Some(Self::adc_an(opcode, Reg::B)),
            0x8A => Some(Self::adc_an(opcode, Reg::C)),
            0x8B => Some(Self::adc_an(opcode, Reg::D)),
            0x8C => Some(Self::adc_an(opcode, Reg::E)),
            0x8D => Some(Self::adc_an(opcode, Reg::H)),
            0x8E => Some(Self::adc_an(opcode, Reg::L)),
            0x8F => Some(Self::adc_an(opcode, Reg::HL)),
            0xCE => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::ADC,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // SUB A,n
            // subtracts n from A
            0x97 => Some(Self::sub_an(opcode, Reg::A)),
            0x90 => Some(Self::sub_an(opcode, Reg::B)),
            0x91 => Some(Self::sub_an(opcode, Reg::C)),
            0x92 => Some(Self::sub_an(opcode, Reg::D)),
            0x93 => Some(Self::sub_an(opcode, Reg::E)),
            0x94 => Some(Self::sub_an(opcode, Reg::H)),
            0x95 => Some(Self::sub_an(opcode, Reg::L)),
            0x96 => Some(Self::sub_an(opcode, Reg::HL)),
            0xD6 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::SUB,
                Some(vec![
                    Operand::Register(Reg::A),
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // SBC A,n
            // subtracts n + Carry flag from A
            0x9F => Some(Self::adc_an(opcode, Reg::A)),
            0x98 => Some(Self::adc_an(opcode, Reg::B)),
            0x99 => Some(Self::adc_an(opcode, Reg::C)),
            0x9A => Some(Self::adc_an(opcode, Reg::D)),
            0x9B => Some(Self::adc_an(opcode, Reg::E)),
            0x9C => Some(Self::adc_an(opcode, Reg::H)),
            0x9D => Some(Self::adc_an(opcode, Reg::L)),
            0x9E => Some(Self::adc_an(opcode, Reg::HL)),
            // TODO: what opcode for SBC A,# ?
//            0xCE => Some(InstructionInfo::new(
//                opcode,
//                InstructionMnemonic::ADC,
//                Some(vec![
//                    Operand::Register(Reg::A),
//                    Operand::Immediate16
//                ]),
//                8
//            )),

            _ => None
        }
    }

    fn parse_ld_rr(opcode: u8) -> Option<InstructionInfo> {
        let r1 = match opcode {
            0x78..=0x7F => Some(Reg::A),
            0x40..=0x46 => Some(Reg::B),
            0x48..=0x4F => Some(Reg::C),
            0x50..=0x56 => Some(Reg::D),
            0x58..=0x5F => Some(Reg::E),
            0x60..=0x66 => Some(Reg::H),
            0x68..=0x6F => Some(Reg::L),
            0x70..=0x77 => Some(Reg::HL),
            _ => None
        };

        if r1.is_none() { return None; }
        let r1 = r1.unwrap();

        // get second hex value (f7 -> 7)
        let r2 = match opcode & 0xf {
            0 | 0x8 => Some(Reg::B),
            1 | 0x9 => Some(Reg::C),
            2 | 0xA => Some(Reg::D),
            3 | 0xB => Some(Reg::E),
            4 | 0xC => Some(Reg::H),
            5 | 0xD => Some(Reg::L),
            6 | 0xE => Some(Reg::HL),
            7 | 0xF => Some(Reg::A),
            _ => None
        };

        if let Some(r2) = r2 {
            return Some(Self::ld_rr(opcode, r1, r2));
        }
        return None;
    }

    fn ld_rr(opcode: u8, r1: Reg, r2: Reg) -> InstructionInfo {
        let cycle_count = if r1.is16bit() || r2.is16bit() { 8 } else { 4 };
        let r1 = if r1.is16bit() {
            Operand::Address(Addr::Register(r1))
        } else {
            Operand::Register(r1)
        };
        let r2 = if r2.is16bit() {
            ValueType::Address(Addr::Register(r2))
        } else {
            ValueType::Register(r2)
        };

        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![r1, Operand::Value(r2)]),
            cycle_count
        )
    }

    fn ld_rn(opcode: u8, register: Reg) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 12 } else { 8 };
        let op = if register == Reg::HL {
            Operand::Address(Addr::Register(Reg::HL))
        } else {
            Operand::Register(register)
        };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                op,
                Operand::Value(ValueType::Immediate)
            ]),
            cycle_count
        )
    }

    fn ld_rn16(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(register),
                Operand::Value(ValueType::Address(Addr::Immediate))
            ]),
            16
        )
    }

    fn ld_n16r(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Address(Addr::Immediate),
                Operand::Value(ValueType::Register(register))
            ]),
            16
        )
    }

    fn ld_r16n16(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(register),
                Operand::Value(ValueType::Immediate16)
            ]),
            12
        )
    }

    fn push(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::PUSH,
            Some(vec![
                Operand::Value(ValueType::Register(register))
            ]),
            16
        )
    }

    fn pop(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::POP,
            Some(vec![
                Operand::Register(register)
            ]),
            12
        )
    }

    fn nop(opcode: u8) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::NOP,
            None,
            0
        )
    }

    fn halt(opcode: u8) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::HALT,
            None,
            0 // TODO: how many cycles for HALT?
        )
    }

    fn add_an(opcode: u8, register: Reg) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 8 } else { 4 };
        let op = if register == Reg::HL {
            ValueType::Address(Addr::Register(Reg::HL))
        } else {
            ValueType::Register(register)
        };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::ADD,
            Some(vec![
                Operand::Register(Reg::A),
                Operand::Value(op)
            ]),
            cycle_count
        )
    }

    fn adc_an(opcode: u8, register: Reg) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 8 } else { 4 };
        let op = if register == Reg::HL {
            ValueType::Address(Addr::Register(Reg::HL))
        } else {
            ValueType::Register(register)
        };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::ADC,
            Some(vec![
                Operand::Register(Reg::A),
                Operand::Value(op)
            ]),
            cycle_count
        )
    }

    fn sub_an(opcode: u8, register: Reg) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 8 } else { 4 };
        let op = if register == Reg::HL {
            ValueType::Address(Addr::Register(Reg::HL))
        } else {
            ValueType::Register(register)
        };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::SUB,
            Some(vec![
                Operand::Register(Reg::A),
                Operand::Value(op)
            ]),
            cycle_count
        )
    }

    fn sbc_an(opcode: u8, register: Reg) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 8 } else { 4 };
        let op = if register == Reg::HL {
            ValueType::Address(Addr::Register(Reg::HL))
        } else {
            ValueType::Register(register)
        };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::SBC,
            Some(vec![
                Operand::Register(Reg::A),
                Operand::Value(op)
            ]),
            cycle_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage() {
        let mut covered = Vec::new();
        for i in 0..0xff {
            if let Some(_) = Decoder::decode_opcode(i) {
                covered.push(i);
            }
        }
        assert_eq!(covered.len(), 0xff - 11);
    }
}