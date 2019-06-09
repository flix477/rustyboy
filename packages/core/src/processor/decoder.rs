use crate::processor::flag_register::Flag;
use crate::processor::instruction::AddressType as Addr;
use crate::processor::instruction::Reference as Ref;
use crate::processor::instruction::*;
use crate::processor::registers::RegisterType as Reg;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8, prefix: Prefix) -> Option<InstructionInfo> {
        if let Prefix::CB = prefix {
            return Self::decode_cb_opcode(opcode);
        }

        match opcode {
            // NOP
            0 => Some(InstructionInfo::new(opcode, Mnemonic::NOP, None, 4)),

            // CB prefix
            0xCB => Some(InstructionInfo::new(opcode, Mnemonic::CB, None, 0)),

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
            0x76 => Some(InstructionInfo::new(opcode, Mnemonic::HALT, None, 4)),

            // LD r1,r2
            // put value r2 in r1
            0x40..=0x75 | 0x77..=0x7F => Self::parse_ld_rr(opcode),

            // LD A,n
            // put value n into A
            0x0A => Some(Self::ld_rr(opcode, Reg::A, Reg::BC)),
            0x1A => Some(Self::ld_rr(opcode, Reg::A, Reg::DE)),
            0xFA => Some(Self::ld_rn16(opcode, Reg::A)),
            0x3E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Immediate),
                ]),
                8,
            )),

            // LD n,A
            0x02 => Some(Self::ld_rr(opcode, Reg::BC, Reg::A)),
            0x12 => Some(Self::ld_rr(opcode, Reg::DE, Reg::A)),
            0xEA => Some(Self::ld_n16r(opcode, Reg::A)),

            // LD A,(C)
            // Put value at address $FF00 + register C into A
            0xF2 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::IncRegister(Reg::C))),
                ]),
                8,
            )),

            // LD (C),A
            // Put A into address $FF00 + register C
            0xE2 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::IncRegister(Reg::C))),
                    Operand::Value(ValueType::Register(Reg::A)),
                ]),
                8,
            )),

            // LDD A,(HL)
            0x3A => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LDD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL))),
                ]),
                8,
            )),

            // LDD (HL),A
            0x32 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LDD,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                    Operand::Value(ValueType::Register(Reg::A)),
                ]),
                8,
            )),

            // LDI A,(HL)
            0x2A => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LDI,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL))),
                ]),
                8,
            )),

            // LDI (HL),A
            0x22 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LDI,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                    Operand::Value(ValueType::Register(Reg::A)),
                ]),
                8,
            )),

            // LDH (n),A
            0xE0 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::IncImmediate)),
                    Operand::Value(ValueType::Register(Reg::A)),
                ]),
                12,
            )),

            // LDH A,(n)
            0xF0 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::IncImmediate)),
                ]),
                12,
            )),

            // LD n,nn 16bit
            0x01 => Some(Self::ld_r16n16(opcode, Reg::BC)),
            0x11 => Some(Self::ld_r16n16(opcode, Reg::DE)),
            0x21 => Some(Self::ld_r16n16(opcode, Reg::HL)),
            0x31 => Some(Self::ld_r16n16(opcode, Reg::SP)),

            // LD SP,HL
            0xF9 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::SP)),
                    Operand::Value(ValueType::Register(Reg::HL)),
                ]),
                8,
            )),

            // LDHL SP,n
            0xF8 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LDHL,
                Some(vec![
                    Operand::Value(ValueType::Register(Reg::SP)),
                    Operand::Value(ValueType::Immediate),
                ]),
                12,
            )),

            // LD (nn),SP
            0x08 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::LD,
                Some(vec![
                    Operand::Reference(Ref::Address(Addr::Immediate)),
                    Operand::Value(ValueType::Register(Reg::SP)),
                ]),
                20,
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
            0x86 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::ADD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Address(Addr::Register(Reg::HL))),
                ]),
                8,
            )),
            0xC6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::ADD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::A)),
                    Operand::Value(ValueType::Immediate),
                ]),
                8,
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
                Mnemonic::ADC,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::SUB,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::SBC,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::AND,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::OR,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::XOR,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::CP,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
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
                Mnemonic::INC,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                12,
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
                Mnemonic::DEC,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                12,
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
                Mnemonic::ADD,
                Some(vec![
                    Operand::Reference(Ref::Register(Reg::SP)),
                    Operand::Value(ValueType::SignedImmediate),
                ]),
                16,
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
            0x27 => Some(InstructionInfo::new(opcode, Mnemonic::DAA, None, 4)),

            // CPL
            // Complement register A (flip all bits)
            0x2F => Some(InstructionInfo::new(opcode, Mnemonic::CPL, None, 4)),

            // CCF
            // Complement carry flag (toggle it)
            0x3F => Some(InstructionInfo::new(opcode, Mnemonic::CCF, None, 4)),

            // SCF
            // Set carry flag
            0x37 => Some(InstructionInfo::new(opcode, Mnemonic::SCF, None, 4)),

            // STOP
            0x10 => Some(InstructionInfo::new(opcode, Mnemonic::STOP, None, 4)),

            // DI
            0xF3 => Some(InstructionInfo::new(opcode, Mnemonic::DI, None, 4)),

            // EI
            0xFB => Some(InstructionInfo::new(opcode, Mnemonic::EI, None, 4)),

            // RLCA
            0x07 => Some(InstructionInfo::new(opcode, Mnemonic::RLCA, None, 4)),

            // RLA
            0x17 => Some(InstructionInfo::new(opcode, Mnemonic::RLA, None, 4)),

            // RRCA
            0x0F => Some(InstructionInfo::new(opcode, Mnemonic::RRCA, None, 4)),

            // RRA
            0x1F => Some(InstructionInfo::new(opcode, Mnemonic::RRA, None, 4)),

            // JP nn
            0xC3 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::JP,
                Some(vec![Operand::Value(ValueType::Immediate16)]),
                12,
            )),

            // JP cc,nn
            0xC2 => Some(Self::jp(opcode, (Flag::Zero, false))),
            0xCA => Some(Self::jp(opcode, (Flag::Zero, true))),
            0xD2 => Some(Self::jp(opcode, (Flag::Carry, false))),
            0xDA => Some(Self::jp(opcode, (Flag::Carry, true))),

            // JP (HL)
            0xE9 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::JP,
                Some(vec![Operand::Value(ValueType::Register(Reg::HL))]),
                4,
            )),

            // JR n
            0x18 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::JR,
                Some(vec![Operand::Value(ValueType::Immediate)]),
                8,
            )),

            // JR cc,nn
            0x20 => Some(Self::jr(opcode, (Flag::Zero, false))),
            0x28 => Some(Self::jr(opcode, (Flag::Zero, true))),
            0x30 => Some(Self::jr(opcode, (Flag::Carry, false))),
            0x38 => Some(Self::jr(opcode, (Flag::Carry, true))),

            // CALL nn
            0xCD => Some(InstructionInfo::new(
                opcode,
                Mnemonic::CALL,
                Some(vec![Operand::Value(ValueType::Immediate16)]),
                12,
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
            0xC9 => Some(InstructionInfo::new(opcode, Mnemonic::RET, None, 8)),

            // RET cc
            0xC0 => Some(Self::ret(opcode, (Flag::Zero, false))),
            0xC8 => Some(Self::ret(opcode, (Flag::Zero, true))),
            0xD0 => Some(Self::ret(opcode, (Flag::Carry, false))),
            0xD8 => Some(Self::ret(opcode, (Flag::Carry, true))),

            // RETI
            0xD9 => Some(InstructionInfo::new(opcode, Mnemonic::RETI, None, 8)),

            _ => None,
        }
    }

    fn decode_cb_opcode(opcode: u8) -> Option<InstructionInfo> {
        match opcode {
            // RLC n
            // Rotate n left
            0x07 => Some(Self::rlc(opcode, Reg::A)),
            0x00 => Some(Self::rlc(opcode, Reg::B)),
            0x01 => Some(Self::rlc(opcode, Reg::C)),
            0x02 => Some(Self::rlc(opcode, Reg::D)),
            0x03 => Some(Self::rlc(opcode, Reg::E)),
            0x04 => Some(Self::rlc(opcode, Reg::H)),
            0x05 => Some(Self::rlc(opcode, Reg::L)),
            0x06 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RLC,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // RL n
            // Rotate n left through carry flag
            0x17 => Some(Self::rl(opcode, Reg::A)),
            0x10 => Some(Self::rl(opcode, Reg::B)),
            0x11 => Some(Self::rl(opcode, Reg::C)),
            0x12 => Some(Self::rl(opcode, Reg::D)),
            0x13 => Some(Self::rl(opcode, Reg::E)),
            0x14 => Some(Self::rl(opcode, Reg::H)),
            0x15 => Some(Self::rl(opcode, Reg::L)),
            0x16 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RL,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // RRC n
            // Rotate n right
            0x0F => Some(Self::rrc(opcode, Reg::A)),
            0x08 => Some(Self::rrc(opcode, Reg::B)),
            0x09 => Some(Self::rrc(opcode, Reg::C)),
            0x0A => Some(Self::rrc(opcode, Reg::D)),
            0x0B => Some(Self::rrc(opcode, Reg::E)),
            0x0C => Some(Self::rrc(opcode, Reg::H)),
            0x0D => Some(Self::rrc(opcode, Reg::L)),
            0x0E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RRC,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // RR n
            // Rotate n right through carry flag
            0x1F => Some(Self::rr(opcode, Reg::A)),
            0x18 => Some(Self::rr(opcode, Reg::B)),
            0x19 => Some(Self::rr(opcode, Reg::C)),
            0x1A => Some(Self::rr(opcode, Reg::D)),
            0x1B => Some(Self::rr(opcode, Reg::E)),
            0x1C => Some(Self::rr(opcode, Reg::H)),
            0x1D => Some(Self::rr(opcode, Reg::L)),
            0x1E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RR,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // SWAP n
            // Swap upper and lower nibble of n
            0x37 => Some(Self::swap(opcode, Reg::A)),
            0x30 => Some(Self::swap(opcode, Reg::B)),
            0x31 => Some(Self::swap(opcode, Reg::C)),
            0x32 => Some(Self::swap(opcode, Reg::D)),
            0x33 => Some(Self::swap(opcode, Reg::E)),
            0x34 => Some(Self::swap(opcode, Reg::H)),
            0x35 => Some(Self::swap(opcode, Reg::L)),
            0x36 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SWAP,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // SLA n
            // Shift n left into carry
            0x27 => Some(Self::sla(opcode, Reg::A)),
            0x20 => Some(Self::sla(opcode, Reg::B)),
            0x21 => Some(Self::sla(opcode, Reg::C)),
            0x22 => Some(Self::sla(opcode, Reg::D)),
            0x23 => Some(Self::sla(opcode, Reg::E)),
            0x24 => Some(Self::sla(opcode, Reg::H)),
            0x25 => Some(Self::sla(opcode, Reg::L)),
            0x26 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SLA,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // SRA n
            // Shift n right into carry, MSB untouched
            0x2F => Some(Self::sra(opcode, Reg::A)),
            0x28 => Some(Self::sra(opcode, Reg::B)),
            0x29 => Some(Self::sra(opcode, Reg::C)),
            0x2A => Some(Self::sra(opcode, Reg::D)),
            0x2B => Some(Self::sra(opcode, Reg::E)),
            0x2C => Some(Self::sra(opcode, Reg::H)),
            0x2D => Some(Self::sra(opcode, Reg::L)),
            0x2E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SRA,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // SRL n
            // Shift n right into carry, MSB set to 0
            0x3F => Some(Self::srl(opcode, Reg::A)),
            0x38 => Some(Self::srl(opcode, Reg::B)),
            0x39 => Some(Self::srl(opcode, Reg::C)),
            0x3A => Some(Self::srl(opcode, Reg::D)),
            0x3B => Some(Self::srl(opcode, Reg::E)),
            0x3C => Some(Self::srl(opcode, Reg::H)),
            0x3D => Some(Self::srl(opcode, Reg::L)),
            0x3E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SRL,
                Some(vec![Operand::Reference(Ref::Address(Addr::Register(
                    Reg::HL,
                )))]),
                16,
            )),

            // BIT b,r
            // Test bit b in r
            0x47 => Some(Self::bit(opcode, 0, Reg::A)),
            0x40 => Some(Self::bit(opcode, 0, Reg::B)),
            0x41 => Some(Self::bit(opcode, 0, Reg::C)),
            0x42 => Some(Self::bit(opcode, 0, Reg::D)),
            0x43 => Some(Self::bit(opcode, 0, Reg::E)),
            0x44 => Some(Self::bit(opcode, 0, Reg::H)),
            0x45 => Some(Self::bit(opcode, 0, Reg::L)),
            0x46 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(0)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x4F => Some(Self::bit(opcode, 1, Reg::A)),
            0x48 => Some(Self::bit(opcode, 1, Reg::B)),
            0x49 => Some(Self::bit(opcode, 1, Reg::C)),
            0x4A => Some(Self::bit(opcode, 1, Reg::D)),
            0x4B => Some(Self::bit(opcode, 1, Reg::E)),
            0x4C => Some(Self::bit(opcode, 1, Reg::H)),
            0x4D => Some(Self::bit(opcode, 1, Reg::L)),
            0x4E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(1)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x57 => Some(Self::bit(opcode, 2, Reg::A)),
            0x50 => Some(Self::bit(opcode, 2, Reg::B)),
            0x51 => Some(Self::bit(opcode, 2, Reg::C)),
            0x52 => Some(Self::bit(opcode, 2, Reg::D)),
            0x53 => Some(Self::bit(opcode, 2, Reg::E)),
            0x54 => Some(Self::bit(opcode, 2, Reg::H)),
            0x55 => Some(Self::bit(opcode, 2, Reg::L)),
            0x56 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(2)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x5F => Some(Self::bit(opcode, 3, Reg::A)),
            0x58 => Some(Self::bit(opcode, 3, Reg::B)),
            0x59 => Some(Self::bit(opcode, 3, Reg::C)),
            0x5A => Some(Self::bit(opcode, 3, Reg::D)),
            0x5B => Some(Self::bit(opcode, 3, Reg::E)),
            0x5C => Some(Self::bit(opcode, 3, Reg::H)),
            0x5D => Some(Self::bit(opcode, 3, Reg::L)),
            0x5E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(3)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x67 => Some(Self::bit(opcode, 4, Reg::A)),
            0x60 => Some(Self::bit(opcode, 4, Reg::B)),
            0x61 => Some(Self::bit(opcode, 4, Reg::C)),
            0x62 => Some(Self::bit(opcode, 4, Reg::D)),
            0x63 => Some(Self::bit(opcode, 4, Reg::E)),
            0x64 => Some(Self::bit(opcode, 4, Reg::H)),
            0x65 => Some(Self::bit(opcode, 4, Reg::L)),
            0x66 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(4)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x6F => Some(Self::bit(opcode, 5, Reg::A)),
            0x68 => Some(Self::bit(opcode, 5, Reg::B)),
            0x69 => Some(Self::bit(opcode, 5, Reg::C)),
            0x6A => Some(Self::bit(opcode, 5, Reg::D)),
            0x6B => Some(Self::bit(opcode, 5, Reg::E)),
            0x6C => Some(Self::bit(opcode, 5, Reg::H)),
            0x6D => Some(Self::bit(opcode, 5, Reg::L)),
            0x6E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(5)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x77 => Some(Self::bit(opcode, 6, Reg::A)),
            0x70 => Some(Self::bit(opcode, 6, Reg::B)),
            0x71 => Some(Self::bit(opcode, 6, Reg::C)),
            0x72 => Some(Self::bit(opcode, 6, Reg::D)),
            0x73 => Some(Self::bit(opcode, 6, Reg::E)),
            0x74 => Some(Self::bit(opcode, 6, Reg::H)),
            0x75 => Some(Self::bit(opcode, 6, Reg::L)),
            0x76 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(6)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x7F => Some(Self::bit(opcode, 7, Reg::A)),
            0x78 => Some(Self::bit(opcode, 7, Reg::B)),
            0x79 => Some(Self::bit(opcode, 7, Reg::C)),
            0x7A => Some(Self::bit(opcode, 7, Reg::D)),
            0x7B => Some(Self::bit(opcode, 7, Reg::E)),
            0x7C => Some(Self::bit(opcode, 7, Reg::H)),
            0x7D => Some(Self::bit(opcode, 7, Reg::L)),
            0x7E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::BIT,
                Some(vec![
                    Operand::Value(ValueType::Constant(7)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            // SET b,r
            // Set bit b in r
            0xC7 => Some(Self::set(opcode, 0, Reg::A)),
            0xC0 => Some(Self::set(opcode, 0, Reg::B)),
            0xC1 => Some(Self::set(opcode, 0, Reg::C)),
            0xC2 => Some(Self::set(opcode, 0, Reg::D)),
            0xC3 => Some(Self::set(opcode, 0, Reg::E)),
            0xC4 => Some(Self::set(opcode, 0, Reg::H)),
            0xC5 => Some(Self::set(opcode, 0, Reg::L)),
            0xC6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(0)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xCF => Some(Self::set(opcode, 1, Reg::A)),
            0xC8 => Some(Self::set(opcode, 1, Reg::B)),
            0xC9 => Some(Self::set(opcode, 1, Reg::C)),
            0xCA => Some(Self::set(opcode, 1, Reg::D)),
            0xCB => Some(Self::set(opcode, 1, Reg::E)),
            0xCC => Some(Self::set(opcode, 1, Reg::H)),
            0xCD => Some(Self::set(opcode, 1, Reg::L)),
            0xCE => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(1)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xD7 => Some(Self::set(opcode, 2, Reg::A)),
            0xD0 => Some(Self::set(opcode, 2, Reg::B)),
            0xD1 => Some(Self::set(opcode, 2, Reg::C)),
            0xD2 => Some(Self::set(opcode, 2, Reg::D)),
            0xD3 => Some(Self::set(opcode, 2, Reg::E)),
            0xD4 => Some(Self::set(opcode, 2, Reg::H)),
            0xD5 => Some(Self::set(opcode, 2, Reg::L)),
            0xD6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(2)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xDF => Some(Self::set(opcode, 3, Reg::A)),
            0xD8 => Some(Self::set(opcode, 3, Reg::B)),
            0xD9 => Some(Self::set(opcode, 3, Reg::C)),
            0xDA => Some(Self::set(opcode, 3, Reg::D)),
            0xDB => Some(Self::set(opcode, 3, Reg::E)),
            0xDC => Some(Self::set(opcode, 3, Reg::H)),
            0xDD => Some(Self::set(opcode, 3, Reg::L)),
            0xDE => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(3)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xE7 => Some(Self::set(opcode, 4, Reg::A)),
            0xE0 => Some(Self::set(opcode, 4, Reg::B)),
            0xE1 => Some(Self::set(opcode, 4, Reg::C)),
            0xE2 => Some(Self::set(opcode, 4, Reg::D)),
            0xE3 => Some(Self::set(opcode, 4, Reg::E)),
            0xE4 => Some(Self::set(opcode, 4, Reg::H)),
            0xE5 => Some(Self::set(opcode, 4, Reg::L)),
            0xE6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(4)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xEF => Some(Self::set(opcode, 5, Reg::A)),
            0xE8 => Some(Self::set(opcode, 5, Reg::B)),
            0xE9 => Some(Self::set(opcode, 5, Reg::C)),
            0xEA => Some(Self::set(opcode, 5, Reg::D)),
            0xEB => Some(Self::set(opcode, 5, Reg::E)),
            0xEC => Some(Self::set(opcode, 5, Reg::H)),
            0xED => Some(Self::set(opcode, 5, Reg::L)),
            0xEE => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(5)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xF7 => Some(Self::set(opcode, 6, Reg::A)),
            0xF0 => Some(Self::set(opcode, 6, Reg::B)),
            0xF1 => Some(Self::set(opcode, 6, Reg::C)),
            0xF2 => Some(Self::set(opcode, 6, Reg::D)),
            0xF3 => Some(Self::set(opcode, 6, Reg::E)),
            0xF4 => Some(Self::set(opcode, 6, Reg::H)),
            0xF5 => Some(Self::set(opcode, 6, Reg::L)),
            0xF6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(6)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xFF => Some(Self::set(opcode, 7, Reg::A)),
            0xF8 => Some(Self::set(opcode, 7, Reg::B)),
            0xF9 => Some(Self::set(opcode, 7, Reg::C)),
            0xFA => Some(Self::set(opcode, 7, Reg::D)),
            0xFB => Some(Self::set(opcode, 7, Reg::E)),
            0xFC => Some(Self::set(opcode, 7, Reg::H)),
            0xFD => Some(Self::set(opcode, 7, Reg::L)),
            0xFE => Some(InstructionInfo::new(
                opcode,
                Mnemonic::SET,
                Some(vec![
                    Operand::Value(ValueType::Constant(7)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            // RES b,r
            // Reset bit b in r
            0x87 => Some(Self::res(opcode, 0, Reg::A)),
            0x80 => Some(Self::res(opcode, 0, Reg::B)),
            0x81 => Some(Self::res(opcode, 0, Reg::C)),
            0x82 => Some(Self::res(opcode, 0, Reg::D)),
            0x83 => Some(Self::res(opcode, 0, Reg::E)),
            0x84 => Some(Self::res(opcode, 0, Reg::H)),
            0x85 => Some(Self::res(opcode, 0, Reg::L)),
            0x86 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(0)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x8F => Some(Self::res(opcode, 1, Reg::A)),
            0x88 => Some(Self::res(opcode, 1, Reg::B)),
            0x89 => Some(Self::res(opcode, 1, Reg::C)),
            0x8A => Some(Self::res(opcode, 1, Reg::D)),
            0x8B => Some(Self::res(opcode, 1, Reg::E)),
            0x8C => Some(Self::res(opcode, 1, Reg::H)),
            0x8D => Some(Self::res(opcode, 1, Reg::L)),
            0x8E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(1)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x97 => Some(Self::res(opcode, 2, Reg::A)),
            0x90 => Some(Self::res(opcode, 2, Reg::B)),
            0x91 => Some(Self::res(opcode, 2, Reg::C)),
            0x92 => Some(Self::res(opcode, 2, Reg::D)),
            0x93 => Some(Self::res(opcode, 2, Reg::E)),
            0x94 => Some(Self::res(opcode, 2, Reg::H)),
            0x95 => Some(Self::res(opcode, 2, Reg::L)),
            0x96 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(2)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0x9F => Some(Self::res(opcode, 3, Reg::A)),
            0x98 => Some(Self::res(opcode, 3, Reg::B)),
            0x99 => Some(Self::res(opcode, 3, Reg::C)),
            0x9A => Some(Self::res(opcode, 3, Reg::D)),
            0x9B => Some(Self::res(opcode, 3, Reg::E)),
            0x9C => Some(Self::res(opcode, 3, Reg::H)),
            0x9D => Some(Self::res(opcode, 3, Reg::L)),
            0x9E => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(3)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xA7 => Some(Self::res(opcode, 4, Reg::A)),
            0xA0 => Some(Self::res(opcode, 4, Reg::B)),
            0xA1 => Some(Self::res(opcode, 4, Reg::C)),
            0xA2 => Some(Self::res(opcode, 4, Reg::D)),
            0xA3 => Some(Self::res(opcode, 4, Reg::E)),
            0xA4 => Some(Self::res(opcode, 4, Reg::H)),
            0xA5 => Some(Self::res(opcode, 4, Reg::L)),
            0xA6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(4)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xAF => Some(Self::res(opcode, 5, Reg::A)),
            0xA8 => Some(Self::res(opcode, 5, Reg::B)),
            0xA9 => Some(Self::res(opcode, 5, Reg::C)),
            0xAA => Some(Self::res(opcode, 5, Reg::D)),
            0xAB => Some(Self::res(opcode, 5, Reg::E)),
            0xAC => Some(Self::res(opcode, 5, Reg::H)),
            0xAD => Some(Self::res(opcode, 5, Reg::L)),
            0xAE => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(5)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xB7 => Some(Self::res(opcode, 6, Reg::A)),
            0xB0 => Some(Self::res(opcode, 6, Reg::B)),
            0xB1 => Some(Self::res(opcode, 6, Reg::C)),
            0xB2 => Some(Self::res(opcode, 6, Reg::D)),
            0xB3 => Some(Self::res(opcode, 6, Reg::E)),
            0xB4 => Some(Self::res(opcode, 6, Reg::H)),
            0xB5 => Some(Self::res(opcode, 6, Reg::L)),
            0xB6 => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(6)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),

            0xBF => Some(Self::res(opcode, 7, Reg::A)),
            0xB8 => Some(Self::res(opcode, 7, Reg::B)),
            0xB9 => Some(Self::res(opcode, 7, Reg::C)),
            0xBA => Some(Self::res(opcode, 7, Reg::D)),
            0xBB => Some(Self::res(opcode, 7, Reg::E)),
            0xBC => Some(Self::res(opcode, 7, Reg::H)),
            0xBD => Some(Self::res(opcode, 7, Reg::L)),
            0xBE => Some(InstructionInfo::new(
                opcode,
                Mnemonic::RES,
                Some(vec![
                    Operand::Value(ValueType::Constant(7)),
                    Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
                ]),
                16,
            )),
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
            _ => None,
        };

        r1?;
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
            _ => None,
        };

        if let Some(r2) = r2 {
            return Some(Self::ld_rr(opcode, r1, r2));
        }
        None
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
            Mnemonic::LD,
            Some(vec![Operand::Reference(r1), Operand::Value(r2)]),
            cycle_count,
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
            Mnemonic::LD,
            Some(vec![
                Operand::Reference(op),
                Operand::Value(ValueType::Immediate),
            ]),
            cycle_count,
        )
    }

    fn ld_rn16(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::LD,
            Some(vec![
                Operand::Reference(Ref::Register(register)),
                Operand::Value(ValueType::Address(Addr::Immediate)),
            ]),
            16,
        )
    }

    fn ld_n16r(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::LD,
            Some(vec![
                Operand::Reference(Ref::Address(Addr::Immediate)),
                Operand::Value(ValueType::Register(register)),
            ]),
            16,
        )
    }

    fn ld_r16n16(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::LD,
            Some(vec![
                Operand::Reference(Ref::Register(register)),
                Operand::Value(ValueType::Immediate16),
            ]),
            12,
        )
    }

    fn push(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::PUSH,
            Some(vec![Operand::Value(ValueType::Register(register))]),
            16,
        )
    }

    fn pop(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::POP,
            Some(vec![Operand::Reference(Ref::Register(register))]),
            12,
        )
    }

    fn add_an(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::ADD,
            Some(vec![
                Operand::Reference(Ref::Register(Reg::A)),
                Operand::Value(ValueType::Register(register)),
            ]),
            4,
        )
    }

    fn adc_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::ADC)
    }

    fn sub_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::SUB)
    }

    fn sbc_an(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::SBC)
    }

    fn and(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::AND)
    }

    fn or(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::OR)
    }

    fn xor(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::XOR)
    }

    fn cp(opcode: u8, register: Reg) -> InstructionInfo {
        Self::alu(opcode, register, Mnemonic::CP)
    }

    fn inc(opcode: u8, register: Reg) -> InstructionInfo {
        Self::incdec(opcode, register, Mnemonic::INC)
    }

    fn dec(opcode: u8, register: Reg) -> InstructionInfo {
        Self::incdec(opcode, register, Mnemonic::DEC)
    }

    fn incdec(opcode: u8, register: Reg, mnemonic: Mnemonic) -> InstructionInfo {
        let cycle_count = if register.is16bit() { 8 } else { 4 };
        InstructionInfo::new(
            opcode,
            mnemonic,
            Some(vec![Operand::Reference(Ref::Register(register))]),
            cycle_count,
        )
    }

    fn alu(opcode: u8, register: Reg, mnemonic: Mnemonic) -> InstructionInfo {
        let cycle_count = if register == Reg::HL { 8 } else { 4 };
        let op = if register == Reg::HL {
            ValueType::Address(Addr::Register(Reg::HL))
        } else {
            ValueType::Register(register)
        };
        InstructionInfo::new(
            opcode,
            mnemonic,
            Some(vec![Operand::Value(op)]),
            cycle_count,
        )
    }

    fn add_hl(opcode: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::ADD,
            Some(vec![
                Operand::Reference(Ref::Register(Reg::HL)),
                Operand::Value(ValueType::Register(register)),
            ]),
            8,
        )
    }

    fn rlc(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::RLC)
    }

    fn rl(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::RL)
    }

    fn rrc(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::RRC)
    }

    fn rr(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::RR)
    }

    fn swap(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::SWAP)
    }

    fn sla(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::SLA)
    }

    fn sra(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::SRA)
    }

    fn srl(opcode: u8, register: Reg) -> InstructionInfo {
        Self::rotates(opcode, register, Mnemonic::SRL)
    }

    fn bit(opcode: u8, bit: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::BIT,
            Some(vec![
                Operand::Value(ValueType::Constant(u16::from(bit))),
                Operand::Reference(Ref::Register(register)),
            ]),
            8,
        )
    }

    fn set(opcode: u8, bit: u8, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::SET,
            Some(vec![
                Operand::Value(ValueType::Constant(u16::from(bit))),
                Operand::Reference(Ref::Register(register)),
            ]),
            8,
        )
    }

    fn res(opcode: u8, bit: u16, register: Reg) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::RES,
            Some(vec![
                Operand::Value(ValueType::Constant(bit as u16)),
                Operand::Reference(Ref::Register(register)),
            ]),
            8,
        )
    }

    fn rotates(opcode: u8, register: Reg, mnemonic: Mnemonic) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            mnemonic,
            Some(vec![Operand::Reference(Ref::Register(register))]),
            8,
        )
    }

    fn jp(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::JP,
            Some(vec![
                Operand::Condition(condition),
                Operand::Value(ValueType::Immediate16),
            ]),
            12,
        )
    }

    fn jr(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::JR,
            Some(vec![
                Operand::Condition(condition),
                Operand::Value(ValueType::Immediate),
            ]),
            8,
        )
    }

    fn call(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::CALL,
            Some(vec![
                Operand::Condition(condition),
                Operand::Value(ValueType::Immediate16),
            ]),
            12,
        )
    }

    fn rst(opcode: u8, address: u16) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::RST,
            Some(vec![Operand::Value(ValueType::Constant(address))]),
            32,
        )
    }

    fn ret(opcode: u8, condition: (Flag, bool)) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            Mnemonic::RET,
            Some(vec![Operand::Condition(condition)]),
            8,
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
            0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
        ];
        let empty: Vec<&str> = Vec::new();
        for i in 0..0xff {
            if !empty_instr.contains(&i) && Decoder::decode_opcode(i, Prefix::None).is_none() {
                not_covered.push(format!("{:X}", i));
            }
        }
        assert_eq!(not_covered, empty);
    }
}
