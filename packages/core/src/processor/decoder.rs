use crate::processor::instruction::AddressType as Addr;
use crate::processor::instruction::Reference as Ref;
use crate::processor::instruction::*;
use crate::processor::registers::flag_register::Flag;
use crate::processor::registers::RegisterType as Reg;

/// Decodes a GameBoy opcode and returns information about the corresponding instruction
pub fn decode_opcode(opcode: u8, prefix: Prefix) -> Option<InstructionInfo> {
    if let Prefix::CB = prefix {
        return decode_cb_opcode(opcode);
    }

    match opcode {
        // NOP
        0 => Some(InstructionInfo::new(Mnemonic::NOP, 4)),

        // CB prefix
        0xCB => Some(InstructionInfo::new(Mnemonic::CB, 0)),

        // LD nn,n
        // put value nn into n
        // TODO: diverging docs, nn->n or n->nn?
        0x06 => Some(ld_rn(Reg::B)),
        0x0E => Some(ld_rn(Reg::C)),
        0x16 => Some(ld_rn(Reg::D)),
        0x1E => Some(ld_rn(Reg::E)),
        0x26 => Some(ld_rn(Reg::H)),
        0x2E => Some(ld_rn(Reg::L)),
        0x36 => Some(ld_rn(Reg::HL)),

        // HALT
        0x76 => Some(InstructionInfo::new(Mnemonic::HALT, 4)),

        // LD r1,r2
        // put value r2 in r1
        0x40..=0x75 | 0x77..=0x7F => parse_ld_rr(opcode),

        // LD A,n
        // put value n into A
        0x0A => Some(ld_rr(Reg::A, Reg::BC)),
        0x1A => Some(ld_rr(Reg::A, Reg::DE)),
        0xFA => Some(ld_rn16(Reg::A)),
        0x3E => Some(InstructionInfo::new(
            Mnemonic::LD(Ref::Register(Reg::A), ValueType::Immediate),
            8,
        )),

        // LD n,A
        0x02 => Some(ld_rr(Reg::BC, Reg::A)),
        0x12 => Some(ld_rr(Reg::DE, Reg::A)),
        0xEA => Some(ld_n16r(Reg::A)),

        // LD A,(C)
        // Put value at address $FF00 + register C into A
        0xF2 => Some(InstructionInfo::new(
            Mnemonic::LD(
                Ref::Register(Reg::A),
                ValueType::Address(Addr::IncRegister(Reg::C)),
            ),
            8,
        )),

        // LD (C),A
        // Put A into address $FF00 + register C
        0xE2 => Some(InstructionInfo::new(
            Mnemonic::LD(
                Ref::Address(Addr::IncRegister(Reg::C)),
                ValueType::Register(Reg::A),
            ),
            8,
        )),

        // LDD A,(HL)
        0x3A => Some(InstructionInfo::new(
            Mnemonic::LDD(
                Ref::Register(Reg::A),
                ValueType::Address(Addr::Register(Reg::HL)),
            ),
            8,
        )),

        // LDD (HL),A
        0x32 => Some(InstructionInfo::new(
            Mnemonic::LDD(
                Ref::Address(Addr::Register(Reg::HL)),
                ValueType::Register(Reg::A),
            ),
            8,
        )),

        // LDI A,(HL)
        0x2A => Some(InstructionInfo::new(
            Mnemonic::LDI(
                Ref::Register(Reg::A),
                ValueType::Address(Addr::Register(Reg::HL)),
            ),
            8,
        )),

        // LDI (HL),A
        0x22 => Some(InstructionInfo::new(
            Mnemonic::LDI(
                Ref::Address(Addr::Register(Reg::HL)),
                ValueType::Register(Reg::A),
            ),
            8,
        )),

        // LDH (n),A
        0xE0 => Some(InstructionInfo::new(
            Mnemonic::LD(
                Ref::Address(Addr::IncImmediate),
                ValueType::Register(Reg::A),
            ),
            12,
        )),

        // LDH A,(n)
        0xF0 => Some(InstructionInfo::new(
            Mnemonic::LD(
                Ref::Register(Reg::A),
                ValueType::Address(Addr::IncImmediate),
            ),
            12,
        )),

        // LD n,nn 16bit
        0x01 => Some(ld_r16n16(Reg::BC)),
        0x11 => Some(ld_r16n16(Reg::DE)),
        0x21 => Some(ld_r16n16(Reg::HL)),
        0x31 => Some(ld_r16n16(Reg::SP)),

        // LD SP,HL
        0xF9 => Some(InstructionInfo::new(
            Mnemonic::LD(Ref::Register(Reg::SP), ValueType::Register(Reg::HL)),
            8,
        )),

        // LDHL SP,n
        0xF8 => Some(InstructionInfo::new(Mnemonic::LDHL, 12)),

        // LD (nn),SP
        0x08 => Some(InstructionInfo::new(
            Mnemonic::LD(Ref::Address(Addr::Immediate), ValueType::Register(Reg::SP)),
            20,
        )),

        // PUSH nn
        0xF5 => Some(push(Reg::AF)),
        0xC5 => Some(push(Reg::BC)),
        0xD5 => Some(push(Reg::DE)),
        0xE5 => Some(push(Reg::HL)),

        // POP nn
        0xF1 => Some(pop(Reg::AF)),
        0xC1 => Some(pop(Reg::BC)),
        0xD1 => Some(pop(Reg::DE)),
        0xE1 => Some(pop(Reg::HL)),

        // ADD A,n
        // add n to A
        0x87 => Some(add_an(Reg::A)),
        0x80 => Some(add_an(Reg::B)),
        0x81 => Some(add_an(Reg::C)),
        0x82 => Some(add_an(Reg::D)),
        0x83 => Some(add_an(Reg::E)),
        0x84 => Some(add_an(Reg::H)),
        0x85 => Some(add_an(Reg::L)),
        0x86 => Some(InstructionInfo::new(
            Mnemonic::ADD(Reg::A, ValueType::Address(Addr::Register(Reg::HL))),
            8,
        )),
        0xC6 => Some(InstructionInfo::new(
            Mnemonic::ADD(Reg::A, ValueType::Immediate),
            8,
        )),

        // ADC A,n
        // Add n + Carry flag to A
        0x8F => Some(adc_an(Reg::A)),
        0x88 => Some(adc_an(Reg::B)),
        0x89 => Some(adc_an(Reg::C)),
        0x8A => Some(adc_an(Reg::D)),
        0x8B => Some(adc_an(Reg::E)),
        0x8C => Some(adc_an(Reg::H)),
        0x8D => Some(adc_an(Reg::L)),
        0x8E => Some(adc_an(Reg::HL)),
        0xCE => Some(InstructionInfo::new(Mnemonic::ADC(ValueType::Immediate), 8)),

        // SUB A,n
        // subtracts n from A
        0x97 => Some(sub_an(Reg::A)),
        0x90 => Some(sub_an(Reg::B)),
        0x91 => Some(sub_an(Reg::C)),
        0x92 => Some(sub_an(Reg::D)),
        0x93 => Some(sub_an(Reg::E)),
        0x94 => Some(sub_an(Reg::H)),
        0x95 => Some(sub_an(Reg::L)),
        0x96 => Some(sub_an(Reg::HL)),
        0xD6 => Some(InstructionInfo::new(Mnemonic::SUB(ValueType::Immediate), 8)),

        // SBC A,n
        // subtracts n + Carry flag from A
        0x9F => Some(sbc_an(Reg::A)),
        0x98 => Some(sbc_an(Reg::B)),
        0x99 => Some(sbc_an(Reg::C)),
        0x9A => Some(sbc_an(Reg::D)),
        0x9B => Some(sbc_an(Reg::E)),
        0x9C => Some(sbc_an(Reg::H)),
        0x9D => Some(sbc_an(Reg::L)),
        0x9E => Some(sbc_an(Reg::HL)),
        0xDE => Some(InstructionInfo::new(Mnemonic::SBC(ValueType::Immediate), 8)),

        // AND n
        // Logically AND n with A, result in A
        0xA7 => Some(and(Reg::A)),
        0xA0 => Some(and(Reg::B)),
        0xA1 => Some(and(Reg::C)),
        0xA2 => Some(and(Reg::D)),
        0xA3 => Some(and(Reg::E)),
        0xA4 => Some(and(Reg::H)),
        0xA5 => Some(and(Reg::L)),
        0xA6 => Some(and(Reg::HL)),
        0xE6 => Some(InstructionInfo::new(Mnemonic::AND(ValueType::Immediate), 8)),

        // OR n
        // Logically OR n with A, result in A
        0xB7 => Some(or(Reg::A)),
        0xB0 => Some(or(Reg::B)),
        0xB1 => Some(or(Reg::C)),
        0xB2 => Some(or(Reg::D)),
        0xB3 => Some(or(Reg::E)),
        0xB4 => Some(or(Reg::H)),
        0xB5 => Some(or(Reg::L)),
        0xB6 => Some(or(Reg::HL)),
        0xF6 => Some(InstructionInfo::new(Mnemonic::OR(ValueType::Immediate), 8)),

        // XOR n
        // Logically XOR n with A, result in A
        0xAF => Some(xor(Reg::A)),
        0xA8 => Some(xor(Reg::B)),
        0xA9 => Some(xor(Reg::C)),
        0xAA => Some(xor(Reg::D)),
        0xAB => Some(xor(Reg::E)),
        0xAC => Some(xor(Reg::H)),
        0xAD => Some(xor(Reg::L)),
        0xAE => Some(xor(Reg::HL)),
        0xEE => Some(InstructionInfo::new(Mnemonic::XOR(ValueType::Immediate), 8)),

        // CP n
        // Compare A with n
        0xBF => Some(cp(Reg::A)),
        0xB8 => Some(cp(Reg::B)),
        0xB9 => Some(cp(Reg::C)),
        0xBA => Some(cp(Reg::D)),
        0xBB => Some(cp(Reg::E)),
        0xBC => Some(cp(Reg::H)),
        0xBD => Some(cp(Reg::L)),
        0xBE => Some(cp(Reg::HL)),
        0xFE => Some(InstructionInfo::new(Mnemonic::CP(ValueType::Immediate), 8)),

        // INC n
        // Increment register n
        0x3C => Some(inc(Reg::A)),
        0x04 => Some(inc(Reg::B)),
        0x0C => Some(inc(Reg::C)),
        0x14 => Some(inc(Reg::D)),
        0x1C => Some(inc(Reg::E)),
        0x24 => Some(inc(Reg::H)),
        0x2C => Some(inc(Reg::L)),
        0x34 => Some(InstructionInfo::new(
            Mnemonic::INC(Ref::Address(Addr::Register(Reg::HL))),
            12,
        )),

        // DEC n
        // Decrement register n
        0x3D => Some(dec(Reg::A)),
        0x05 => Some(dec(Reg::B)),
        0x0D => Some(dec(Reg::C)),
        0x15 => Some(dec(Reg::D)),
        0x1D => Some(dec(Reg::E)),
        0x25 => Some(dec(Reg::H)),
        0x2D => Some(dec(Reg::L)),
        0x35 => Some(InstructionInfo::new(
            Mnemonic::DEC(Ref::Address(Addr::Register(Reg::HL))),
            12,
        )),

        // ADD HL,n
        // Add to HL, result in HL
        0x09 => Some(add_hl(Reg::BC)),
        0x19 => Some(add_hl(Reg::DE)),
        0x29 => Some(add_hl(Reg::HL)),
        0x39 => Some(add_hl(Reg::SP)),

        // ADD SP,n
        // Add n to SP, result in SP
        0xE8 => Some(InstructionInfo::new(
            Mnemonic::ADD(Reg::SP, ValueType::SignedImmediate),
            16,
        )),

        // INC nn
        // Increment register nn
        0x03 => Some(inc(Reg::BC)),
        0x13 => Some(inc(Reg::DE)),
        0x23 => Some(inc(Reg::HL)),
        0x33 => Some(inc(Reg::SP)),

        // DEC nn
        // Decrement register nn
        0x0B => Some(dec(Reg::BC)),
        0x1B => Some(dec(Reg::DE)),
        0x2B => Some(dec(Reg::HL)),
        0x3B => Some(dec(Reg::SP)),

        // DAA
        // Decimal adjust register A
        0x27 => Some(InstructionInfo::new(Mnemonic::DAA, 4)),

        // CPL
        // Complement register A (flip all bits)
        0x2F => Some(InstructionInfo::new(Mnemonic::CPL, 4)),

        // CCF
        // Complement carry flag (toggle it)
        0x3F => Some(InstructionInfo::new(Mnemonic::CCF, 4)),

        // SCF
        // Set carry flag
        0x37 => Some(InstructionInfo::new(Mnemonic::SCF, 4)),

        // STOP
        0x10 => Some(InstructionInfo::new(Mnemonic::STOP, 4)),

        // DI
        0xF3 => Some(InstructionInfo::new(Mnemonic::DI, 4)),

        // EI
        0xFB => Some(InstructionInfo::new(Mnemonic::EI, 4)),

        // RLCA
        0x07 => Some(InstructionInfo::new(Mnemonic::RLCA, 4)),

        // RLA
        0x17 => Some(InstructionInfo::new(Mnemonic::RLA, 4)),

        // RRCA
        0x0F => Some(InstructionInfo::new(Mnemonic::RRCA, 4)),

        // RRA
        0x1F => Some(InstructionInfo::new(Mnemonic::RRA, 4)),

        // JP nn
        0xC3 => Some(InstructionInfo::new(
            Mnemonic::JP(None, ValueType::Immediate16),
            12,
        )),

        // JP cc,nn
        0xC2 => Some(jp(Condition(Flag::Zero, false))),
        0xCA => Some(jp(Condition(Flag::Zero, true))),
        0xD2 => Some(jp(Condition(Flag::Carry, false))),
        0xDA => Some(jp(Condition(Flag::Carry, true))),

        // JP (HL)
        0xE9 => Some(InstructionInfo::new(
            Mnemonic::JP(None, ValueType::Register(Reg::HL)),
            4,
        )),

        // JR n
        0x18 => Some(InstructionInfo::new(
            Mnemonic::JR(None, ValueType::Immediate),
            8,
        )),

        // JR cc,nn
        0x20 => Some(jr(Condition(Flag::Zero, false))),
        0x28 => Some(jr(Condition(Flag::Zero, true))),
        0x30 => Some(jr(Condition(Flag::Carry, false))),
        0x38 => Some(jr(Condition(Flag::Carry, true))),

        // CALL nn
        0xCD => Some(InstructionInfo::new(
            Mnemonic::CALL(None, ValueType::Immediate16),
            12,
        )),

        // CALL cc,nn
        0xC4 => Some(call(Condition(Flag::Zero, false))),
        0xCC => Some(call(Condition(Flag::Zero, true))),
        0xD4 => Some(call(Condition(Flag::Carry, false))),
        0xDC => Some(call(Condition(Flag::Carry, true))),

        // RST n
        0xC7 => Some(rst(0x00)),
        0xCF => Some(rst(0x08)),
        0xD7 => Some(rst(0x10)),
        0xDF => Some(rst(0x18)),
        0xE7 => Some(rst(0x20)),
        0xEF => Some(rst(0x28)),
        0xF7 => Some(rst(0x30)),
        0xFF => Some(rst(0x38)),

        // RET
        0xC9 => Some(InstructionInfo::new(Mnemonic::RET(None), 8)),

        // RET cc
        0xC0 => Some(ret(Condition(Flag::Zero, false))),
        0xC8 => Some(ret(Condition(Flag::Zero, true))),
        0xD0 => Some(ret(Condition(Flag::Carry, false))),
        0xD8 => Some(ret(Condition(Flag::Carry, true))),

        // RETI
        0xD9 => Some(InstructionInfo::new(Mnemonic::RETI, 8)),

        _ => None,
    }
}

/// Decodes a GameBoy opcode with a CB prefix
/// and returns information about the corresponding instruction
fn decode_cb_opcode(opcode: u8) -> Option<InstructionInfo> {
    match opcode {
        // RLC n
        // Rotate n left
        0x07 => Some(rlc(Reg::A)),
        0x00 => Some(rlc(Reg::B)),
        0x01 => Some(rlc(Reg::C)),
        0x02 => Some(rlc(Reg::D)),
        0x03 => Some(rlc(Reg::E)),
        0x04 => Some(rlc(Reg::H)),
        0x05 => Some(rlc(Reg::L)),
        0x06 => Some(InstructionInfo::new(
            Mnemonic::RLC(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // RL n
        // Rotate n left through carry flag
        0x17 => Some(rl(Reg::A)),
        0x10 => Some(rl(Reg::B)),
        0x11 => Some(rl(Reg::C)),
        0x12 => Some(rl(Reg::D)),
        0x13 => Some(rl(Reg::E)),
        0x14 => Some(rl(Reg::H)),
        0x15 => Some(rl(Reg::L)),
        0x16 => Some(InstructionInfo::new(
            Mnemonic::RL(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // RRC n
        // Rotate n right
        0x0F => Some(rrc(Reg::A)),
        0x08 => Some(rrc(Reg::B)),
        0x09 => Some(rrc(Reg::C)),
        0x0A => Some(rrc(Reg::D)),
        0x0B => Some(rrc(Reg::E)),
        0x0C => Some(rrc(Reg::H)),
        0x0D => Some(rrc(Reg::L)),
        0x0E => Some(InstructionInfo::new(
            Mnemonic::RRC(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // RR n
        // Rotate n right through carry flag
        0x1F => Some(rr(Reg::A)),
        0x18 => Some(rr(Reg::B)),
        0x19 => Some(rr(Reg::C)),
        0x1A => Some(rr(Reg::D)),
        0x1B => Some(rr(Reg::E)),
        0x1C => Some(rr(Reg::H)),
        0x1D => Some(rr(Reg::L)),
        0x1E => Some(InstructionInfo::new(
            Mnemonic::RR(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // SWAP n
        // Swap upper and lower nibble of n
        0x37 => Some(swap(Reg::A)),
        0x30 => Some(swap(Reg::B)),
        0x31 => Some(swap(Reg::C)),
        0x32 => Some(swap(Reg::D)),
        0x33 => Some(swap(Reg::E)),
        0x34 => Some(swap(Reg::H)),
        0x35 => Some(swap(Reg::L)),
        0x36 => Some(InstructionInfo::new(
            Mnemonic::SWAP(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // SLA n
        // Shift n left into carry
        0x27 => Some(sla(Reg::A)),
        0x20 => Some(sla(Reg::B)),
        0x21 => Some(sla(Reg::C)),
        0x22 => Some(sla(Reg::D)),
        0x23 => Some(sla(Reg::E)),
        0x24 => Some(sla(Reg::H)),
        0x25 => Some(sla(Reg::L)),
        0x26 => Some(InstructionInfo::new(
            Mnemonic::SLA(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // SRA n
        // Shift n right into carry, MSB untouched
        0x2F => Some(sra(Reg::A)),
        0x28 => Some(sra(Reg::B)),
        0x29 => Some(sra(Reg::C)),
        0x2A => Some(sra(Reg::D)),
        0x2B => Some(sra(Reg::E)),
        0x2C => Some(sra(Reg::H)),
        0x2D => Some(sra(Reg::L)),
        0x2E => Some(InstructionInfo::new(
            Mnemonic::SRA(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // SRL n
        // Shift n right into carry, MSB set to 0
        0x3F => Some(srl(Reg::A)),
        0x38 => Some(srl(Reg::B)),
        0x39 => Some(srl(Reg::C)),
        0x3A => Some(srl(Reg::D)),
        0x3B => Some(srl(Reg::E)),
        0x3C => Some(srl(Reg::H)),
        0x3D => Some(srl(Reg::L)),
        0x3E => Some(InstructionInfo::new(
            Mnemonic::SRL(Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // BIT b,r
        // Test bit b in r
        0x47 => Some(bit(0, Reg::A)),
        0x40 => Some(bit(0, Reg::B)),
        0x41 => Some(bit(0, Reg::C)),
        0x42 => Some(bit(0, Reg::D)),
        0x43 => Some(bit(0, Reg::E)),
        0x44 => Some(bit(0, Reg::H)),
        0x45 => Some(bit(0, Reg::L)),
        0x46 => Some(InstructionInfo::new(
            Mnemonic::BIT(0, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x4F => Some(bit(1, Reg::A)),
        0x48 => Some(bit(1, Reg::B)),
        0x49 => Some(bit(1, Reg::C)),
        0x4A => Some(bit(1, Reg::D)),
        0x4B => Some(bit(1, Reg::E)),
        0x4C => Some(bit(1, Reg::H)),
        0x4D => Some(bit(1, Reg::L)),
        0x4E => Some(InstructionInfo::new(
            Mnemonic::BIT(1, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x57 => Some(bit(2, Reg::A)),
        0x50 => Some(bit(2, Reg::B)),
        0x51 => Some(bit(2, Reg::C)),
        0x52 => Some(bit(2, Reg::D)),
        0x53 => Some(bit(2, Reg::E)),
        0x54 => Some(bit(2, Reg::H)),
        0x55 => Some(bit(2, Reg::L)),
        0x56 => Some(InstructionInfo::new(
            Mnemonic::BIT(2, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x5F => Some(bit(3, Reg::A)),
        0x58 => Some(bit(3, Reg::B)),
        0x59 => Some(bit(3, Reg::C)),
        0x5A => Some(bit(3, Reg::D)),
        0x5B => Some(bit(3, Reg::E)),
        0x5C => Some(bit(3, Reg::H)),
        0x5D => Some(bit(3, Reg::L)),
        0x5E => Some(InstructionInfo::new(
            Mnemonic::BIT(3, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x67 => Some(bit(4, Reg::A)),
        0x60 => Some(bit(4, Reg::B)),
        0x61 => Some(bit(4, Reg::C)),
        0x62 => Some(bit(4, Reg::D)),
        0x63 => Some(bit(4, Reg::E)),
        0x64 => Some(bit(4, Reg::H)),
        0x65 => Some(bit(4, Reg::L)),
        0x66 => Some(InstructionInfo::new(
            Mnemonic::BIT(4, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x6F => Some(bit(5, Reg::A)),
        0x68 => Some(bit(5, Reg::B)),
        0x69 => Some(bit(5, Reg::C)),
        0x6A => Some(bit(5, Reg::D)),
        0x6B => Some(bit(5, Reg::E)),
        0x6C => Some(bit(5, Reg::H)),
        0x6D => Some(bit(5, Reg::L)),
        0x6E => Some(InstructionInfo::new(
            Mnemonic::BIT(5, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x77 => Some(bit(6, Reg::A)),
        0x70 => Some(bit(6, Reg::B)),
        0x71 => Some(bit(6, Reg::C)),
        0x72 => Some(bit(6, Reg::D)),
        0x73 => Some(bit(6, Reg::E)),
        0x74 => Some(bit(6, Reg::H)),
        0x75 => Some(bit(6, Reg::L)),
        0x76 => Some(InstructionInfo::new(
            Mnemonic::BIT(6, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x7F => Some(bit(7, Reg::A)),
        0x78 => Some(bit(7, Reg::B)),
        0x79 => Some(bit(7, Reg::C)),
        0x7A => Some(bit(7, Reg::D)),
        0x7B => Some(bit(7, Reg::E)),
        0x7C => Some(bit(7, Reg::H)),
        0x7D => Some(bit(7, Reg::L)),
        0x7E => Some(InstructionInfo::new(
            Mnemonic::BIT(7, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // SET b,r
        // Set bit b in r
        0xC7 => Some(set(0, Reg::A)),
        0xC0 => Some(set(0, Reg::B)),
        0xC1 => Some(set(0, Reg::C)),
        0xC2 => Some(set(0, Reg::D)),
        0xC3 => Some(set(0, Reg::E)),
        0xC4 => Some(set(0, Reg::H)),
        0xC5 => Some(set(0, Reg::L)),
        0xC6 => Some(InstructionInfo::new(
            Mnemonic::SET(0, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xCF => Some(set(1, Reg::A)),
        0xC8 => Some(set(1, Reg::B)),
        0xC9 => Some(set(1, Reg::C)),
        0xCA => Some(set(1, Reg::D)),
        0xCB => Some(set(1, Reg::E)),
        0xCC => Some(set(1, Reg::H)),
        0xCD => Some(set(1, Reg::L)),
        0xCE => Some(InstructionInfo::new(
            Mnemonic::SET(1, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xD7 => Some(set(2, Reg::A)),
        0xD0 => Some(set(2, Reg::B)),
        0xD1 => Some(set(2, Reg::C)),
        0xD2 => Some(set(2, Reg::D)),
        0xD3 => Some(set(2, Reg::E)),
        0xD4 => Some(set(2, Reg::H)),
        0xD5 => Some(set(2, Reg::L)),
        0xD6 => Some(InstructionInfo::new(
            Mnemonic::SET(2, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xDF => Some(set(3, Reg::A)),
        0xD8 => Some(set(3, Reg::B)),
        0xD9 => Some(set(3, Reg::C)),
        0xDA => Some(set(3, Reg::D)),
        0xDB => Some(set(3, Reg::E)),
        0xDC => Some(set(3, Reg::H)),
        0xDD => Some(set(3, Reg::L)),
        0xDE => Some(InstructionInfo::new(
            Mnemonic::SET(3, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xE7 => Some(set(4, Reg::A)),
        0xE0 => Some(set(4, Reg::B)),
        0xE1 => Some(set(4, Reg::C)),
        0xE2 => Some(set(4, Reg::D)),
        0xE3 => Some(set(4, Reg::E)),
        0xE4 => Some(set(4, Reg::H)),
        0xE5 => Some(set(4, Reg::L)),
        0xE6 => Some(InstructionInfo::new(
            Mnemonic::SET(4, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xEF => Some(set(5, Reg::A)),
        0xE8 => Some(set(5, Reg::B)),
        0xE9 => Some(set(5, Reg::C)),
        0xEA => Some(set(5, Reg::D)),
        0xEB => Some(set(5, Reg::E)),
        0xEC => Some(set(5, Reg::H)),
        0xED => Some(set(5, Reg::L)),
        0xEE => Some(InstructionInfo::new(
            Mnemonic::SET(5, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xF7 => Some(set(6, Reg::A)),
        0xF0 => Some(set(6, Reg::B)),
        0xF1 => Some(set(6, Reg::C)),
        0xF2 => Some(set(6, Reg::D)),
        0xF3 => Some(set(6, Reg::E)),
        0xF4 => Some(set(6, Reg::H)),
        0xF5 => Some(set(6, Reg::L)),
        0xF6 => Some(InstructionInfo::new(
            Mnemonic::SET(6, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xFF => Some(set(7, Reg::A)),
        0xF8 => Some(set(7, Reg::B)),
        0xF9 => Some(set(7, Reg::C)),
        0xFA => Some(set(7, Reg::D)),
        0xFB => Some(set(7, Reg::E)),
        0xFC => Some(set(7, Reg::H)),
        0xFD => Some(set(7, Reg::L)),
        0xFE => Some(InstructionInfo::new(
            Mnemonic::SET(7, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        // RES b,r
        // Reset bit b in r
        0x87 => Some(res(0, Reg::A)),
        0x80 => Some(res(0, Reg::B)),
        0x81 => Some(res(0, Reg::C)),
        0x82 => Some(res(0, Reg::D)),
        0x83 => Some(res(0, Reg::E)),
        0x84 => Some(res(0, Reg::H)),
        0x85 => Some(res(0, Reg::L)),
        0x86 => Some(InstructionInfo::new(
            Mnemonic::RES(0, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x8F => Some(res(1, Reg::A)),
        0x88 => Some(res(1, Reg::B)),
        0x89 => Some(res(1, Reg::C)),
        0x8A => Some(res(1, Reg::D)),
        0x8B => Some(res(1, Reg::E)),
        0x8C => Some(res(1, Reg::H)),
        0x8D => Some(res(1, Reg::L)),
        0x8E => Some(InstructionInfo::new(
            Mnemonic::RES(1, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x97 => Some(res(2, Reg::A)),
        0x90 => Some(res(2, Reg::B)),
        0x91 => Some(res(2, Reg::C)),
        0x92 => Some(res(2, Reg::D)),
        0x93 => Some(res(2, Reg::E)),
        0x94 => Some(res(2, Reg::H)),
        0x95 => Some(res(2, Reg::L)),
        0x96 => Some(InstructionInfo::new(
            Mnemonic::RES(2, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0x9F => Some(res(3, Reg::A)),
        0x98 => Some(res(3, Reg::B)),
        0x99 => Some(res(3, Reg::C)),
        0x9A => Some(res(3, Reg::D)),
        0x9B => Some(res(3, Reg::E)),
        0x9C => Some(res(3, Reg::H)),
        0x9D => Some(res(3, Reg::L)),
        0x9E => Some(InstructionInfo::new(
            Mnemonic::RES(3, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xA7 => Some(res(4, Reg::A)),
        0xA0 => Some(res(4, Reg::B)),
        0xA1 => Some(res(4, Reg::C)),
        0xA2 => Some(res(4, Reg::D)),
        0xA3 => Some(res(4, Reg::E)),
        0xA4 => Some(res(4, Reg::H)),
        0xA5 => Some(res(4, Reg::L)),
        0xA6 => Some(InstructionInfo::new(
            Mnemonic::RES(4, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xAF => Some(res(5, Reg::A)),
        0xA8 => Some(res(5, Reg::B)),
        0xA9 => Some(res(5, Reg::C)),
        0xAA => Some(res(5, Reg::D)),
        0xAB => Some(res(5, Reg::E)),
        0xAC => Some(res(5, Reg::H)),
        0xAD => Some(res(5, Reg::L)),
        0xAE => Some(InstructionInfo::new(
            Mnemonic::RES(5, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xB7 => Some(res(6, Reg::A)),
        0xB0 => Some(res(6, Reg::B)),
        0xB1 => Some(res(6, Reg::C)),
        0xB2 => Some(res(6, Reg::D)),
        0xB3 => Some(res(6, Reg::E)),
        0xB4 => Some(res(6, Reg::H)),
        0xB5 => Some(res(6, Reg::L)),
        0xB6 => Some(InstructionInfo::new(
            Mnemonic::RES(6, Ref::Address(Addr::Register(Reg::HL))),
            16,
        )),

        0xBF => Some(res(7, Reg::A)),
        0xB8 => Some(res(7, Reg::B)),
        0xB9 => Some(res(7, Reg::C)),
        0xBA => Some(res(7, Reg::D)),
        0xBB => Some(res(7, Reg::E)),
        0xBC => Some(res(7, Reg::H)),
        0xBD => Some(res(7, Reg::L)),
        0xBE => Some(InstructionInfo::new(
            Mnemonic::RES(7, Ref::Address(Addr::Register(Reg::HL))),
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
        return Some(ld_rr(r1, r2));
    }
    None
}

fn ld_rr(r1: Reg, r2: Reg) -> InstructionInfo {
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

    InstructionInfo::new(Mnemonic::LD(r1, r2), cycle_count)
}

fn ld_rn(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 12 } else { 8 };
    let op = if register == Reg::HL {
        Ref::Address(Addr::Register(Reg::HL))
    } else {
        Ref::Register(register)
    };
    InstructionInfo::new(Mnemonic::LD(op, ValueType::Immediate), cycle_count)
}

fn ld_rn16(register: Reg) -> InstructionInfo {
    InstructionInfo::new(
        Mnemonic::LD(Ref::Register(register), ValueType::Address(Addr::Immediate)),
        16,
    )
}

fn ld_n16r(register: Reg) -> InstructionInfo {
    InstructionInfo::new(
        Mnemonic::LD(Ref::Address(Addr::Immediate), ValueType::Register(register)),
        16,
    )
}

fn ld_r16n16(register: Reg) -> InstructionInfo {
    InstructionInfo::new(
        Mnemonic::LD(Ref::Register(register), ValueType::Immediate16),
        12,
    )
}

fn push(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::PUSH(ValueType::Register(register)), 16)
}

fn pop(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::POP(register), 12)
}

fn add_an(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::ADD(Reg::A, ValueType::Register(register)), 4)
}

fn adc_an(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::ADC(op), cycle_count)
}

fn sub_an(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::SUB(op), cycle_count)
}

fn sbc_an(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::SBC(op), cycle_count)
}

fn and(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::AND(op), cycle_count)
}

fn or(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::OR(op), cycle_count)
}

fn xor(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::XOR(op), cycle_count)
}

fn cp(register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionInfo::new(Mnemonic::CP(op), cycle_count)
}

fn inc(register: Reg) -> InstructionInfo {
    let cycle_count = if register.is16bit() { 8 } else { 4 };
    InstructionInfo::new(Mnemonic::INC(Ref::Register(register)), cycle_count)
}

fn dec(register: Reg) -> InstructionInfo {
    let cycle_count = if register.is16bit() { 8 } else { 4 };
    InstructionInfo::new(Mnemonic::DEC(Ref::Register(register)), cycle_count)
}

fn add_hl(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::ADD(Reg::HL, ValueType::Register(register)), 8)
}

fn rlc(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RLC(Ref::Register(register)), 8)
}

fn rl(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RL(Ref::Register(register)), 8)
}

fn rrc(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RRC(Ref::Register(register)), 8)
}

fn rr(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RR(Ref::Register(register)), 8)
}

fn swap(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::SWAP(Ref::Register(register)), 8)
}

fn sla(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::SLA(Ref::Register(register)), 8)
}

fn sra(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::SRA(Ref::Register(register)), 8)
}

fn srl(register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::SRL(Ref::Register(register)), 8)
}

fn bit(bit: u8, register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::BIT(u16::from(bit), Ref::Register(register)), 8)
}

fn set(bit: u8, register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::SET(u16::from(bit), Ref::Register(register)), 8)
}

fn res(bit: u8, register: Reg) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RES(u16::from(bit), Ref::Register(register)), 8)
}

fn jp(condition: Condition) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::JP(Some(condition), ValueType::Immediate16), 12)
}

fn jr(condition: Condition) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::JR(Some(condition), ValueType::Immediate), 8)
}

fn call(condition: Condition) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::CALL(Some(condition), ValueType::Immediate16), 12)
}

fn rst(address: u16) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RST(address), 32)
}

fn ret(condition: Condition) -> InstructionInfo {
    InstructionInfo::new(Mnemonic::RET(Some(condition)), 8)
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
            if !empty_instr.contains(&i) && decode_opcode(i, Prefix::None).is_none() {
                not_covered.push(format!("{:X}", i));
            }
        }
        assert_eq!(not_covered, empty);
    }
}
