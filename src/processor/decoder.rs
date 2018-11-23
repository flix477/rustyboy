use processor::instruction::*;
use processor::instruction::AddressType as Addr;
use processor::instruction::Reference as Ref;
use processor::registers::RegisterType as Reg;
use processor::flag_register::Flag;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8) -> Option<InstructionInfo> {
        match opcode {
            // NOP
            0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::NOP,
                None,
                0
            )),

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
            0x76 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::HALT,
                None,
                4
            )),

            // LD r1,r2
            // put value r2 in r1
            0x40..=0x7F => Self::parse_ld_rr(opcode),

            // LD A,n
            // put value n into A
            0x0A => Some(Self::ld_rr(opcode, Reg::A, Reg::BC)),
            0x1A => Some(Self::ld_rr(opcode, Reg::A, Reg::DE)),
            0xFA => Some(Self::ld_rn16(opcode, Reg::A)),
            0x3E => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

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
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(
                        ValueType::Address(Addr::IncRegister(Reg::C))
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
                    Operand::Reference(
                        Ref::Address(Addr::IncRegister(Reg::C))
                    ),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                8
            )),

            // LDD A,(HL)
            0x3A => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL)))
                ]),
                8
            )),

            // LDD (HL),A
            0x32 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDD,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                8
            )),

            // LDI A,(HL)
            0x2A => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDI,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL)))
                ]),
                8
            )),

            // LDI (HL),A
            0x22 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDI,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                8
            )),

            // LDH (n),A
            0xE0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::IncImmediate)),
                    Operand::Value(ValueType::Register(Reg::A))
                ]),
                12
            )),

            // LDH A,(n)
            0xF0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(
                        ValueType::Address(Addr::IncImmediate)
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
                    Operand::Reference(Ref::Register(Reg::SP)),
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
                    Operand::Reference(Ref::Address(Addr::Immediate)),
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
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // ADC A,n
            // Add n + Carry flag to A
            0x8F => Some(Self::adc_an(opcode, Reg::A)),
            0x88 => Some(Self::adc_an(opcode, Reg::B)),
            0x89 => Some(Self::adc_an(opcode, Reg::C)),
            0x8A => Some(Self::adc_an(opcode, Reg::D)),
            0x8B => Some(Self::adc_an(opcode, Reg::E)),
            0x8C => Some(Self::adc_an(opcode, Reg::H)),
            0x8D => Some(Self::adc_an(opcode, Reg::L)),
            0x8E => Some(Self::adc_an(opcode, Reg::HL)),
            0xCE => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::ADC,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
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
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // SBC A,n
            // subtracts n + Carry flag from A
            0x9F => Some(Self::sbc_an(opcode, Reg::A)),
            0x98 => Some(Self::sbc_an(opcode, Reg::B)),
            0x99 => Some(Self::sbc_an(opcode, Reg::C)),
            0x9A => Some(Self::sbc_an(opcode, Reg::D)),
            0x9B => Some(Self::sbc_an(opcode, Reg::E)),
            0x9C => Some(Self::sbc_an(opcode, Reg::H)),
            0x9D => Some(Self::sbc_an(opcode, Reg::L)),
            0x9E => Some(Self::sbc_an(opcode, Reg::HL)),
            0xDE => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::SBC,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // AND n
            // Logically AND n with A, result in A
            0xA7 => Some(Self::and(opcode, Reg::A)),
            0xA0 => Some(Self::and(opcode, Reg::B)),
            0xA1 => Some(Self::and(opcode, Reg::C)),
            0xA2 => Some(Self::and(opcode, Reg::D)),
            0xA3 => Some(Self::and(opcode, Reg::E)),
            0xA4 => Some(Self::and(opcode, Reg::H)),
            0xA5 => Some(Self::and(opcode, Reg::L)),
            0xA6 => Some(Self::and(opcode, Reg::HL)),
            0xE6 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::AND,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // OR n
            // Logically OR n with A, result in A
            0xB7 => Some(Self::or(opcode, Reg::A)),
            0xB0 => Some(Self::or(opcode, Reg::B)),
            0xB1 => Some(Self::or(opcode, Reg::C)),
            0xB2 => Some(Self::or(opcode, Reg::D)),
            0xB3 => Some(Self::or(opcode, Reg::E)),
            0xB4 => Some(Self::or(opcode, Reg::H)),
            0xB5 => Some(Self::or(opcode, Reg::L)),
            0xB6 => Some(Self::or(opcode, Reg::HL)),
            0xF6 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::OR,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // XOR n
            // Logically XOR n with A, result in A
            0xAF => Some(Self::xor(opcode, Reg::A)),
            0xA8 => Some(Self::xor(opcode, Reg::B)),
            0xA9 => Some(Self::xor(opcode, Reg::C)),
            0xAA => Some(Self::xor(opcode, Reg::D)),
            0xAB => Some(Self::xor(opcode, Reg::E)),
            0xAC => Some(Self::xor(opcode, Reg::H)),
            0xAD => Some(Self::xor(opcode, Reg::L)),
            0xAE => Some(Self::xor(opcode, Reg::HL)),
            0xEE => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::XOR,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // CP n
            // Compare A with n
            0xBF => Some(Self::cp(opcode, Reg::A)),
            0xB8 => Some(Self::cp(opcode, Reg::B)),
            0xB9 => Some(Self::cp(opcode, Reg::C)),
            0xBA => Some(Self::cp(opcode, Reg::D)),
            0xBB => Some(Self::cp(opcode, Reg::E)),
            0xBC => Some(Self::cp(opcode, Reg::H)),
            0xBD => Some(Self::cp(opcode, Reg::L)),
            0xBE => Some(Self::cp(opcode, Reg::HL)),
            0xFE => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::CP,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // INC n
            // Increment register n
            0x3C => Some(Self::inc(opcode, Reg::A)),
            0x04 => Some(Self::inc(opcode, Reg::B)),
            0x0C => Some(Self::inc(opcode, Reg::C)),
            0x14 => Some(Self::inc(opcode, Reg::D)),
            0x1C => Some(Self::inc(opcode, Reg::E)),
            0x24 => Some(Self::inc(opcode, Reg::H)),
            0x2C => Some(Self::inc(opcode, Reg::L)),
            0x34 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::INC,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Immediate))
                ]),
                12
            )),

            // DEC n
            // Decrement register n
            0x3D => Some(Self::dec(opcode, Reg::A)),
            0x05 => Some(Self::dec(opcode, Reg::B)),
            0x0D => Some(Self::dec(opcode, Reg::C)),
            0x15 => Some(Self::dec(opcode, Reg::D)),
            0x1D => Some(Self::dec(opcode, Reg::E)),
            0x25 => Some(Self::dec(opcode, Reg::H)),
            0x2D => Some(Self::dec(opcode, Reg::L)),
            0x35 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::DEC,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Immediate))
                ]),
                12
            )),

            // ADD HL,n
            // Add to HL, result in HL
            0x09 => Some(Self::add_hl(opcode, Reg::BC)),
            0x19 => Some(Self::add_hl(opcode, Reg::DE)),
            0x29 => Some(Self::add_hl(opcode, Reg::HL)),
            0x39 => Some(Self::add_hl(opcode, Reg::SP)),

            // ADD SP,n
            // Add n to SP, result in SP
            0xE8 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::ADD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::SP)),
                    Operand::Value(ValueType::Immediate)
                ]),
                16
            )),

            // INC nn
            // Increment register nn
            0x03 => Some(Self::inc(opcode, Reg::BC)),
            0x13 => Some(Self::inc(opcode, Reg::DE)),
            0x23 => Some(Self::inc(opcode, Reg::HL)),
            0x33 => Some(Self::inc(opcode, Reg::SP)),

            // DEC nn
            // Decrement register nn
            0x0B => Some(Self::dec(opcode, Reg::BC)),
            0x1B => Some(Self::dec(opcode, Reg::DE)),
            0x2B => Some(Self::dec(opcode, Reg::HL)),
            0x3B => Some(Self::dec(opcode, Reg::SP)),

            // DAA
            // Decimal adjust register A
            0x27 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::DAA,
                None,
                4
            )),

            // CPL
            // Complement register A (flip all bits)
            0x2F => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::CPL,
                None,
                4
            )),

            // CCF
            // Complement carry flag (toggle it)
            0x3F => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::CCF,
                None,
                4
            )),

            // SCF
            // Set carry flag
            0x37 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::SCF,
                None,
                4
            )),

            // STOP
            0x10 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::STOP,
                None,
                4
            )),

            // DI
            0xF3 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::DI,
                None,
                4
            )),

            // EI
            0xFB => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::EI,
                None,
                4
            )),

            // RLCA
            0x07 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::RLCA,
                None,
                4
            )),

            // RLA
            0x17 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::RLA,
                None,
                4
            )),

            // RRCA
            0x0F => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::RRCA,
                None,
                4
            )),

            // RRA
            0x1F => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::RRA,
                None,
                4
            )),

            // TODO: Lots of CB opcodes

            // JP nn
            0xC3 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::JP,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                12
            )),

            // JP cc,nn
            0xC2 => Some(Self::jp(opcode, (Flag::Zero, false))),
            0xCA => Some(Self::jp(opcode, (Flag::Zero, true))),
            0xD2 => Some(Self::jp(opcode, (Flag::Carry, false))),
            0xDA => Some(Self::jp(opcode, (Flag::Carry, true))),

            // JP (HL)
            0xE9 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::JP,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL)))
                ]),
                4
            )),

            // JR n
            0x18 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::JR,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                8
            )),

            // JR cc,nn
            0x20 => Some(Self::jr(opcode, (Flag::Zero, false))),
            0x28 => Some(Self::jr(opcode, (Flag::Zero, true))),
            0x30 => Some(Self::jr(opcode, (Flag::Carry, false))),
            0x38 => Some(Self::jr(opcode, (Flag::Carry, true))),

            // CALL nn
            0xCD => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::CALL,
                Some(vec![
                    Operand::Value(ValueType::Address(Addr::Immediate))
                ]),
                12
            )),

            // CALL cc,nn
            0xC4 => Some(Self::call(opcode, (Flag::Zero, false))),
            0xCC => Some(Self::call(opcode, (Flag::Zero, true))),
            0xD4 => Some(Self::call(opcode, (Flag::Carry, false))),
            0xDC => Some(Self::call(opcode, (Flag::Carry, true))),

            // RST n
            0xC7 => Some(Self::rst(opcode, 0x00)),
            0xCF => Some(Self::rst(opcode, 0x08)),
            0xD7 => Some(Self::rst(opcode, 0x10)),
            0xDF => Some(Self::rst(opcode, 0x18)),
            0xE7 => Some(Self::rst(opcode, 0x20)),
            0xEF => Some(Self::rst(opcode, 0x28)),
            0xF7 => Some(Self::rst(opcode, 0x30)),
            0xFF => Some(Self::rst(opcode, 0x38)),

            // RET
            0xC9 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::RET,
                None,
                8
            )),

            // RET cc
            0xC0 => Some(Self::ret(opcode, (Flag::Zero, false))),
            0xC8 => Some(Self::ret(opcode, (Flag::Zero, true))),
            0xD0 => Some(Self::ret(opcode, (Flag::Carry, false))),
            0xD8 => Some(Self::ret(opcode, (Flag::Carry, true))),

            // RETI
            0xD9 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::RETI,
                None,
                8
            )),

            _ => None
        }
    }

    fn parse_ld_rr(opcode: u8) -> Option<InstructionInfo> {
        let r1 = match opcode {
            0x78..=0x7F => Some(Reg::A),
            0x40..=0x47 => Some(Reg::B),
            0x48..=0x4F => Some(Reg::C),
            0x50..=0x57 => Some(Reg::D),
            0x58..=0x5F => Some(Reg::E),
            0x60..=0x67 => Some(Reg::H),
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
            Ref::Address(Addr::Register(r1))
        } else {
            Ref::Register(r1)
        };
        let r2 = if r2.is16bit() {
            ValueType::Address(Addr::Register(r2))
        } else {
            ValueType::Register(r2)
        };

        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![Operand::Reference(r1), Operand::Value(r2)]),
            cycle_count
        )
    }

    fn ld_rn(opcode: u8, register: Reg) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 12 } else { 8 };
        let op = if register == Reg::HL {
            Ref::Address(Addr::Register(Reg::HL))
        } else {
            Ref::Register(register)
        };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Reference(op),
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
                Operand::Reference(Ref::Register(register)),
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
                Operand::Reference(Ref::Address(Addr::Immediate)),
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
                Operand::Reference(Ref::Register(register)),
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
                Operand::Reference(Ref::Register(register))
            ]),
            12
        )
    }

    fn add_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::ADD)
    }

    fn adc_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::ADC)
    }

    fn sub_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::SUB)
    }

    fn sbc_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::SBC)
    }

    fn and(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::AND)
    }

    fn or(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::OR)
    }

    fn xor(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::XOR)
    }

    fn cp(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, InstructionMnemonic::CP)
    }

    fn inc(opcode: u8, register: Reg) -> InstructionInfo {
        Self::incdec(opcode, register, InstructionMnemonic::INC)
    }

    fn dec(opcode: u8, register: Reg) -> InstructionInfo {
        Self::incdec(opcode, register, InstructionMnemonic::DEC)
    }

    fn incdec(opcode: u8, register: Reg, mnemonic: InstructionMnemonic)
        -> InstructionInfo
    {
        let cycle_count = if register.is16bit() { 8 } else { 4 };
        InstructionInfo::new(
            opcode,
            mnemonic,
            Some(vec![
                Operand::Reference(Ref::Register(register))
            ]),
            cycle_count
        )
    }

    fn alu(opcode: u8, register: Reg, mnemonic: InstructionMnemonic)
           -> InstructionInfo
    {
        let cycle_count = if register == Reg::HL { 8 } else { 4 };
        let op = if register == Reg::HL {
            ValueType::Address(Addr::Register(Reg::HL))
        } else {
            ValueType::Register(register)
        };
        InstructionInfo::new(
            opcode,
            mnemonic,
            Some(vec![
                Operand::Reference(Ref::Register(Reg::A)),
                Operand::Value(op)
            ]),
            cycle_count
        )
    }

    fn add_hl(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::ADD,
            Some(vec![
                Operand::Reference(Ref::Register(Reg::HL)),
                Operand::Value(ValueType::Register(register))
            ]),
            8
        )
    }

    fn rlc(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::RLC,
            Some(vec![
                Operand::Reference(Ref::Register(register))
            ]),
            8
        )
    }

    fn jp(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::JP,
            Some(vec![
                Operand::Condition(condition),
                Operand::Value(ValueType::Address(Addr::Immediate))
            ]),
            12
        )
    }

    fn jr(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::JP,
            Some(vec![
                Operand::Condition(condition),
                Operand::Value(ValueType::Address(Addr::Immediate))
            ]),
            8
        )
    }

    fn call(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::CALL,
            Some(vec![
                Operand::Condition(condition),
                Operand::Value(ValueType::Address(Addr::Immediate))
            ]),
            12
        )
    }

    fn rst(opcode: u8, address: u16) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::RST,
            Some(vec![
                Operand::Value(ValueType::Constant(address))
            ]),
            32
        )
    }

    fn ret(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::RET,
            Some(vec![
                Operand::Condition(condition)
            ]),
            8
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage() {
        let mut not_covered = Vec::new();
        let empty_instr = vec![
            0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD
        ];
        let empty: Vec<&str> = Vec::new();
        for i in 0..0xff {
            if !empty_instr.contains(&i) &&
                Decoder::decode_opcode(i).is_none()
            {
                not_covered.push(format!("{:X}", i));
            }
        }
        assert_eq!(not_covered, empty);
    }
}