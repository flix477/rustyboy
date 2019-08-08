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
        0 => Some(InstructionBuilder::new(opcode, Mnemonic::NOP, 4).none()),

        // CB prefix
        0xCB => Some(InstructionBuilder::new(opcode, Mnemonic::CB, 0).none()),

        // LD nn,n
        // put value nn into n
        // TODO: diverging docs, nn->n or n->nn?
        0x06 => Some(ld_rn(opcode, Reg::B)),
        0x0E => Some(ld_rn(opcode, Reg::C)),
        0x16 => Some(ld_rn(opcode, Reg::D)),
        0x1E => Some(ld_rn(opcode, Reg::E)),
        0x26 => Some(ld_rn(opcode, Reg::H)),
        0x2E => Some(ld_rn(opcode, Reg::L)),
        0x36 => Some(ld_rn(opcode, Reg::HL)),

        // HALT
        0x76 => Some(InstructionBuilder::new(opcode, Mnemonic::HALT, 4).none()),

        // LD r1,r2
        // put value r2 in r1
        0x40..=0x75 | 0x77..=0x7F => parse_ld_rr(opcode),

        // LD A,n
        // put value n into A
        0x0A => Some(ld_rr(opcode, Reg::A, Reg::BC)),
        0x1A => Some(ld_rr(opcode, Reg::A, Reg::DE)),
        0xFA => Some(ld_rn16(opcode, Reg::A)),
        0x3E => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 8).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Immediate),
        )),

        // LD n,A
        0x02 => Some(ld_rr(opcode, Reg::BC, Reg::A)),
        0x12 => Some(ld_rr(opcode, Reg::DE, Reg::A)),
        0xEA => Some(ld_n16r(opcode, Reg::A)),

        // LD A,(C)
        // Put value at address $FF00 + register C into A
        0xF2 => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 8).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Address(Addr::IncRegister(Reg::C))),
        )),

        // LD (C),A
        // Put A into address $FF00 + register C
        0xE2 => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 8).args(
            Operand::Reference(Ref::Address(Addr::IncRegister(Reg::C))),
            Operand::Value(ValueType::Register(Reg::A)),
        )),

        // LDD A,(HL)
        0x3A => Some(InstructionBuilder::new(opcode, Mnemonic::LDD, 8).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Address(Addr::Register(Reg::HL))),
        )),

        // LDD (HL),A
        0x32 => Some(InstructionBuilder::new(opcode, Mnemonic::LDD, 8).args(
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
            Operand::Value(ValueType::Register(Reg::A)),
        )),

        // LDI A,(HL)
        0x2A => Some(InstructionBuilder::new(opcode, Mnemonic::LDI, 8).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Address(Addr::Register(Reg::HL))),
        )),

        // LDI (HL),A
        0x22 => Some(InstructionBuilder::new(opcode, Mnemonic::LDI, 8).args(
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
            Operand::Value(ValueType::Register(Reg::A)),
        )),

        // LDH (n),A
        0xE0 => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 12).args(
            Operand::Reference(Ref::Address(Addr::IncImmediate)),
            Operand::Value(ValueType::Register(Reg::A)),
        )),

        // LDH A,(n)
        0xF0 => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 12).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Address(Addr::IncImmediate)),
        )),

        // LD n,nn 16bit
        0x01 => Some(ld_r16n16(opcode, Reg::BC)),
        0x11 => Some(ld_r16n16(opcode, Reg::DE)),
        0x21 => Some(ld_r16n16(opcode, Reg::HL)),
        0x31 => Some(ld_r16n16(opcode, Reg::SP)),

        // LD SP,HL
        0xF9 => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 8).args(
            Operand::Reference(Ref::Register(Reg::SP)),
            Operand::Value(ValueType::Register(Reg::HL)),
        )),

        // LDHL SP,n
        0xF8 => Some(InstructionBuilder::new(opcode, Mnemonic::LDHL, 12).args(
            Operand::Value(ValueType::Register(Reg::SP)),
            Operand::Value(ValueType::Immediate),
        )),

        // LD (nn),SP
        0x08 => Some(InstructionBuilder::new(opcode, Mnemonic::LD, 20).args(
            Operand::Reference(Ref::Address(Addr::Immediate)),
            Operand::Value(ValueType::Register(Reg::SP)),
        )),

        // PUSH nn
        0xF5 => Some(push(opcode, Reg::AF)),
        0xC5 => Some(push(opcode, Reg::BC)),
        0xD5 => Some(push(opcode, Reg::DE)),
        0xE5 => Some(push(opcode, Reg::HL)),

        // POP nn
        0xF1 => Some(pop(opcode, Reg::AF)),
        0xC1 => Some(pop(opcode, Reg::BC)),
        0xD1 => Some(pop(opcode, Reg::DE)),
        0xE1 => Some(pop(opcode, Reg::HL)),

        // ADD A,n
        // add n to A
        0x87 => Some(add_an(opcode, Reg::A)),
        0x80 => Some(add_an(opcode, Reg::B)),
        0x81 => Some(add_an(opcode, Reg::C)),
        0x82 => Some(add_an(opcode, Reg::D)),
        0x83 => Some(add_an(opcode, Reg::E)),
        0x84 => Some(add_an(opcode, Reg::H)),
        0x85 => Some(add_an(opcode, Reg::L)),
        0x86 => Some(InstructionBuilder::new(opcode, Mnemonic::ADD, 8).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Address(Addr::Register(Reg::HL))),
        )),
        0xC6 => Some(InstructionBuilder::new(opcode, Mnemonic::ADD, 8).args(
            Operand::Reference(Ref::Register(Reg::A)),
            Operand::Value(ValueType::Immediate),
        )),

        // ADC A,n
        // Add n + Carry flag to A
        0x8F => Some(adc_an(opcode, Reg::A)),
        0x88 => Some(adc_an(opcode, Reg::B)),
        0x89 => Some(adc_an(opcode, Reg::C)),
        0x8A => Some(adc_an(opcode, Reg::D)),
        0x8B => Some(adc_an(opcode, Reg::E)),
        0x8C => Some(adc_an(opcode, Reg::H)),
        0x8D => Some(adc_an(opcode, Reg::L)),
        0x8E => Some(adc_an(opcode, Reg::HL)),
        0xCE => Some(
            InstructionBuilder::new(opcode, Mnemonic::ADC, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // SUB A,n
        // subtracts n from A
        0x97 => Some(sub_an(opcode, Reg::A)),
        0x90 => Some(sub_an(opcode, Reg::B)),
        0x91 => Some(sub_an(opcode, Reg::C)),
        0x92 => Some(sub_an(opcode, Reg::D)),
        0x93 => Some(sub_an(opcode, Reg::E)),
        0x94 => Some(sub_an(opcode, Reg::H)),
        0x95 => Some(sub_an(opcode, Reg::L)),
        0x96 => Some(sub_an(opcode, Reg::HL)),
        0xD6 => Some(
            InstructionBuilder::new(opcode, Mnemonic::SUB, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // SBC A,n
        // subtracts n + Carry flag from A
        0x9F => Some(sbc_an(opcode, Reg::A)),
        0x98 => Some(sbc_an(opcode, Reg::B)),
        0x99 => Some(sbc_an(opcode, Reg::C)),
        0x9A => Some(sbc_an(opcode, Reg::D)),
        0x9B => Some(sbc_an(opcode, Reg::E)),
        0x9C => Some(sbc_an(opcode, Reg::H)),
        0x9D => Some(sbc_an(opcode, Reg::L)),
        0x9E => Some(sbc_an(opcode, Reg::HL)),
        0xDE => Some(
            InstructionBuilder::new(opcode, Mnemonic::SBC, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // AND n
        // Logically AND n with A, result in A
        0xA7 => Some(and(opcode, Reg::A)),
        0xA0 => Some(and(opcode, Reg::B)),
        0xA1 => Some(and(opcode, Reg::C)),
        0xA2 => Some(and(opcode, Reg::D)),
        0xA3 => Some(and(opcode, Reg::E)),
        0xA4 => Some(and(opcode, Reg::H)),
        0xA5 => Some(and(opcode, Reg::L)),
        0xA6 => Some(and(opcode, Reg::HL)),
        0xE6 => Some(
            InstructionBuilder::new(opcode, Mnemonic::AND, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // OR n
        // Logically OR n with A, result in A
        0xB7 => Some(or(opcode, Reg::A)),
        0xB0 => Some(or(opcode, Reg::B)),
        0xB1 => Some(or(opcode, Reg::C)),
        0xB2 => Some(or(opcode, Reg::D)),
        0xB3 => Some(or(opcode, Reg::E)),
        0xB4 => Some(or(opcode, Reg::H)),
        0xB5 => Some(or(opcode, Reg::L)),
        0xB6 => Some(or(opcode, Reg::HL)),
        0xF6 => Some(
            InstructionBuilder::new(opcode, Mnemonic::OR, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // XOR n
        // Logically XOR n with A, result in A
        0xAF => Some(xor(opcode, Reg::A)),
        0xA8 => Some(xor(opcode, Reg::B)),
        0xA9 => Some(xor(opcode, Reg::C)),
        0xAA => Some(xor(opcode, Reg::D)),
        0xAB => Some(xor(opcode, Reg::E)),
        0xAC => Some(xor(opcode, Reg::H)),
        0xAD => Some(xor(opcode, Reg::L)),
        0xAE => Some(xor(opcode, Reg::HL)),
        0xEE => Some(
            InstructionBuilder::new(opcode, Mnemonic::XOR, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // CP n
        // Compare A with n
        0xBF => Some(cp(opcode, Reg::A)),
        0xB8 => Some(cp(opcode, Reg::B)),
        0xB9 => Some(cp(opcode, Reg::C)),
        0xBA => Some(cp(opcode, Reg::D)),
        0xBB => Some(cp(opcode, Reg::E)),
        0xBC => Some(cp(opcode, Reg::H)),
        0xBD => Some(cp(opcode, Reg::L)),
        0xBE => Some(cp(opcode, Reg::HL)),
        0xFE => Some(
            InstructionBuilder::new(opcode, Mnemonic::CP, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // INC n
        // Increment register n
        0x3C => Some(inc(opcode, Reg::A)),
        0x04 => Some(inc(opcode, Reg::B)),
        0x0C => Some(inc(opcode, Reg::C)),
        0x14 => Some(inc(opcode, Reg::D)),
        0x1C => Some(inc(opcode, Reg::E)),
        0x24 => Some(inc(opcode, Reg::H)),
        0x2C => Some(inc(opcode, Reg::L)),
        0x34 => Some(
            InstructionBuilder::new(opcode, Mnemonic::INC, 12)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // DEC n
        // Decrement register n
        0x3D => Some(dec(opcode, Reg::A)),
        0x05 => Some(dec(opcode, Reg::B)),
        0x0D => Some(dec(opcode, Reg::C)),
        0x15 => Some(dec(opcode, Reg::D)),
        0x1D => Some(dec(opcode, Reg::E)),
        0x25 => Some(dec(opcode, Reg::H)),
        0x2D => Some(dec(opcode, Reg::L)),
        0x35 => Some(
            InstructionBuilder::new(opcode, Mnemonic::DEC, 12)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // ADD HL,n
        // Add to HL, result in HL
        0x09 => Some(add_hl(opcode, Reg::BC)),
        0x19 => Some(add_hl(opcode, Reg::DE)),
        0x29 => Some(add_hl(opcode, Reg::HL)),
        0x39 => Some(add_hl(opcode, Reg::SP)),

        // ADD SP,n
        // Add n to SP, result in SP
        0xE8 => Some(InstructionBuilder::new(opcode, Mnemonic::ADD, 16).args(
            Operand::Reference(Ref::Register(Reg::SP)),
            Operand::Value(ValueType::SignedImmediate),
        )),

        // INC nn
        // Increment register nn
        0x03 => Some(inc(opcode, Reg::BC)),
        0x13 => Some(inc(opcode, Reg::DE)),
        0x23 => Some(inc(opcode, Reg::HL)),
        0x33 => Some(inc(opcode, Reg::SP)),

        // DEC nn
        // Decrement register nn
        0x0B => Some(dec(opcode, Reg::BC)),
        0x1B => Some(dec(opcode, Reg::DE)),
        0x2B => Some(dec(opcode, Reg::HL)),
        0x3B => Some(dec(opcode, Reg::SP)),

        // DAA
        // Decimal adjust register A
        0x27 => Some(InstructionBuilder::new(opcode, Mnemonic::DAA, 4).none()),

        // CPL
        // Complement register A (flip all bits)
        0x2F => Some(InstructionBuilder::new(opcode, Mnemonic::CPL, 4).none()),

        // CCF
        // Complement carry flag (toggle it)
        0x3F => Some(InstructionBuilder::new(opcode, Mnemonic::CCF, 4).none()),

        // SCF
        // Set carry flag
        0x37 => Some(InstructionBuilder::new(opcode, Mnemonic::SCF, 4).none()),

        // STOP
        0x10 => Some(InstructionBuilder::new(opcode, Mnemonic::STOP, 4).none()),

        // DI
        0xF3 => Some(InstructionBuilder::new(opcode, Mnemonic::DI, 4).none()),

        // EI
        0xFB => Some(InstructionBuilder::new(opcode, Mnemonic::EI, 4).none()),

        // RLCA
        0x07 => Some(InstructionBuilder::new(opcode, Mnemonic::RLCA, 4).none()),

        // RLA
        0x17 => Some(InstructionBuilder::new(opcode, Mnemonic::RLA, 4).none()),

        // RRCA
        0x0F => Some(InstructionBuilder::new(opcode, Mnemonic::RRCA, 4).none()),

        // RRA
        0x1F => Some(InstructionBuilder::new(opcode, Mnemonic::RRA, 4).none()),

        // JP nn
        0xC3 => Some(
            InstructionBuilder::new(opcode, Mnemonic::JP, 12)
                .arg(Operand::Value(ValueType::Immediate16)),
        ),

        // JP cc,nn
        0xC2 => Some(jp(opcode, Condition(Flag::Zero, false))),
        0xCA => Some(jp(opcode, Condition(Flag::Zero, true))),
        0xD2 => Some(jp(opcode, Condition(Flag::Carry, false))),
        0xDA => Some(jp(opcode, Condition(Flag::Carry, true))),

        // JP (HL)
        0xE9 => Some(
            InstructionBuilder::new(opcode, Mnemonic::JP, 4)
                .arg(Operand::Value(ValueType::Register(Reg::HL))),
        ),

        // JR n
        0x18 => Some(
            InstructionBuilder::new(opcode, Mnemonic::JR, 8)
                .arg(Operand::Value(ValueType::Immediate)),
        ),

        // JR cc,nn
        0x20 => Some(jr(opcode, Condition(Flag::Zero, false))),
        0x28 => Some(jr(opcode, Condition(Flag::Zero, true))),
        0x30 => Some(jr(opcode, Condition(Flag::Carry, false))),
        0x38 => Some(jr(opcode, Condition(Flag::Carry, true))),

        // CALL nn
        0xCD => Some(
            InstructionBuilder::new(opcode, Mnemonic::CALL, 12)
                .arg(Operand::Value(ValueType::Immediate16)),
        ),

        // CALL cc,nn
        0xC4 => Some(call(opcode, Condition(Flag::Zero, false))),
        0xCC => Some(call(opcode, Condition(Flag::Zero, true))),
        0xD4 => Some(call(opcode, Condition(Flag::Carry, false))),
        0xDC => Some(call(opcode, Condition(Flag::Carry, true))),

        // RST n
        0xC7 => Some(rst(opcode, 0x00)),
        0xCF => Some(rst(opcode, 0x08)),
        0xD7 => Some(rst(opcode, 0x10)),
        0xDF => Some(rst(opcode, 0x18)),
        0xE7 => Some(rst(opcode, 0x20)),
        0xEF => Some(rst(opcode, 0x28)),
        0xF7 => Some(rst(opcode, 0x30)),
        0xFF => Some(rst(opcode, 0x38)),

        // RET
        0xC9 => Some(InstructionBuilder::new(opcode, Mnemonic::RET, 8).none()),

        // RET cc
        0xC0 => Some(ret(opcode, Condition(Flag::Zero, false))),
        0xC8 => Some(ret(opcode, Condition(Flag::Zero, true))),
        0xD0 => Some(ret(opcode, Condition(Flag::Carry, false))),
        0xD8 => Some(ret(opcode, Condition(Flag::Carry, true))),

        // RETI
        0xD9 => Some(InstructionBuilder::new(opcode, Mnemonic::RETI, 8).none()),

        _ => None,
    }
}

/// Decodes a GameBoy opcode with a CB prefix
/// and returns information about the corresponding instruction
fn decode_cb_opcode(opcode: u8) -> Option<InstructionInfo> {
    match opcode {
        // RLC n
        // Rotate n left
        0x07 => Some(rlc(opcode, Reg::A)),
        0x00 => Some(rlc(opcode, Reg::B)),
        0x01 => Some(rlc(opcode, Reg::C)),
        0x02 => Some(rlc(opcode, Reg::D)),
        0x03 => Some(rlc(opcode, Reg::E)),
        0x04 => Some(rlc(opcode, Reg::H)),
        0x05 => Some(rlc(opcode, Reg::L)),
        0x06 => Some(
            InstructionBuilder::new(opcode, Mnemonic::RLC, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // RL n
        // Rotate n left through carry flag
        0x17 => Some(rl(opcode, Reg::A)),
        0x10 => Some(rl(opcode, Reg::B)),
        0x11 => Some(rl(opcode, Reg::C)),
        0x12 => Some(rl(opcode, Reg::D)),
        0x13 => Some(rl(opcode, Reg::E)),
        0x14 => Some(rl(opcode, Reg::H)),
        0x15 => Some(rl(opcode, Reg::L)),
        0x16 => Some(
            InstructionBuilder::new(opcode, Mnemonic::RL, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // RRC n
        // Rotate n right
        0x0F => Some(rrc(opcode, Reg::A)),
        0x08 => Some(rrc(opcode, Reg::B)),
        0x09 => Some(rrc(opcode, Reg::C)),
        0x0A => Some(rrc(opcode, Reg::D)),
        0x0B => Some(rrc(opcode, Reg::E)),
        0x0C => Some(rrc(opcode, Reg::H)),
        0x0D => Some(rrc(opcode, Reg::L)),
        0x0E => Some(
            InstructionBuilder::new(opcode, Mnemonic::RRC, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // RR n
        // Rotate n right through carry flag
        0x1F => Some(rr(opcode, Reg::A)),
        0x18 => Some(rr(opcode, Reg::B)),
        0x19 => Some(rr(opcode, Reg::C)),
        0x1A => Some(rr(opcode, Reg::D)),
        0x1B => Some(rr(opcode, Reg::E)),
        0x1C => Some(rr(opcode, Reg::H)),
        0x1D => Some(rr(opcode, Reg::L)),
        0x1E => Some(
            InstructionBuilder::new(opcode, Mnemonic::RR, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // SWAP n
        // Swap upper and lower nibble of n
        0x37 => Some(swap(opcode, Reg::A)),
        0x30 => Some(swap(opcode, Reg::B)),
        0x31 => Some(swap(opcode, Reg::C)),
        0x32 => Some(swap(opcode, Reg::D)),
        0x33 => Some(swap(opcode, Reg::E)),
        0x34 => Some(swap(opcode, Reg::H)),
        0x35 => Some(swap(opcode, Reg::L)),
        0x36 => Some(
            InstructionBuilder::new(opcode, Mnemonic::SWAP, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // SLA n
        // Shift n left into carry
        0x27 => Some(sla(opcode, Reg::A)),
        0x20 => Some(sla(opcode, Reg::B)),
        0x21 => Some(sla(opcode, Reg::C)),
        0x22 => Some(sla(opcode, Reg::D)),
        0x23 => Some(sla(opcode, Reg::E)),
        0x24 => Some(sla(opcode, Reg::H)),
        0x25 => Some(sla(opcode, Reg::L)),
        0x26 => Some(
            InstructionBuilder::new(opcode, Mnemonic::SLA, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // SRA n
        // Shift n right into carry, MSB untouched
        0x2F => Some(sra(opcode, Reg::A)),
        0x28 => Some(sra(opcode, Reg::B)),
        0x29 => Some(sra(opcode, Reg::C)),
        0x2A => Some(sra(opcode, Reg::D)),
        0x2B => Some(sra(opcode, Reg::E)),
        0x2C => Some(sra(opcode, Reg::H)),
        0x2D => Some(sra(opcode, Reg::L)),
        0x2E => Some(
            InstructionBuilder::new(opcode, Mnemonic::SRA, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // SRL n
        // Shift n right into carry, MSB set to 0
        0x3F => Some(srl(opcode, Reg::A)),
        0x38 => Some(srl(opcode, Reg::B)),
        0x39 => Some(srl(opcode, Reg::C)),
        0x3A => Some(srl(opcode, Reg::D)),
        0x3B => Some(srl(opcode, Reg::E)),
        0x3C => Some(srl(opcode, Reg::H)),
        0x3D => Some(srl(opcode, Reg::L)),
        0x3E => Some(
            InstructionBuilder::new(opcode, Mnemonic::SRL, 16)
                .arg(Operand::Reference(Ref::Address(Addr::Register(Reg::HL)))),
        ),

        // BIT b,r
        // Test bit b in r
        0x47 => Some(bit(opcode, 0, Reg::A)),
        0x40 => Some(bit(opcode, 0, Reg::B)),
        0x41 => Some(bit(opcode, 0, Reg::C)),
        0x42 => Some(bit(opcode, 0, Reg::D)),
        0x43 => Some(bit(opcode, 0, Reg::E)),
        0x44 => Some(bit(opcode, 0, Reg::H)),
        0x45 => Some(bit(opcode, 0, Reg::L)),
        0x46 => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(0)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x4F => Some(bit(opcode, 1, Reg::A)),
        0x48 => Some(bit(opcode, 1, Reg::B)),
        0x49 => Some(bit(opcode, 1, Reg::C)),
        0x4A => Some(bit(opcode, 1, Reg::D)),
        0x4B => Some(bit(opcode, 1, Reg::E)),
        0x4C => Some(bit(opcode, 1, Reg::H)),
        0x4D => Some(bit(opcode, 1, Reg::L)),
        0x4E => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(1)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x57 => Some(bit(opcode, 2, Reg::A)),
        0x50 => Some(bit(opcode, 2, Reg::B)),
        0x51 => Some(bit(opcode, 2, Reg::C)),
        0x52 => Some(bit(opcode, 2, Reg::D)),
        0x53 => Some(bit(opcode, 2, Reg::E)),
        0x54 => Some(bit(opcode, 2, Reg::H)),
        0x55 => Some(bit(opcode, 2, Reg::L)),
        0x56 => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(2)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x5F => Some(bit(opcode, 3, Reg::A)),
        0x58 => Some(bit(opcode, 3, Reg::B)),
        0x59 => Some(bit(opcode, 3, Reg::C)),
        0x5A => Some(bit(opcode, 3, Reg::D)),
        0x5B => Some(bit(opcode, 3, Reg::E)),
        0x5C => Some(bit(opcode, 3, Reg::H)),
        0x5D => Some(bit(opcode, 3, Reg::L)),
        0x5E => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(3)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x67 => Some(bit(opcode, 4, Reg::A)),
        0x60 => Some(bit(opcode, 4, Reg::B)),
        0x61 => Some(bit(opcode, 4, Reg::C)),
        0x62 => Some(bit(opcode, 4, Reg::D)),
        0x63 => Some(bit(opcode, 4, Reg::E)),
        0x64 => Some(bit(opcode, 4, Reg::H)),
        0x65 => Some(bit(opcode, 4, Reg::L)),
        0x66 => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(4)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x6F => Some(bit(opcode, 5, Reg::A)),
        0x68 => Some(bit(opcode, 5, Reg::B)),
        0x69 => Some(bit(opcode, 5, Reg::C)),
        0x6A => Some(bit(opcode, 5, Reg::D)),
        0x6B => Some(bit(opcode, 5, Reg::E)),
        0x6C => Some(bit(opcode, 5, Reg::H)),
        0x6D => Some(bit(opcode, 5, Reg::L)),
        0x6E => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(5)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x77 => Some(bit(opcode, 6, Reg::A)),
        0x70 => Some(bit(opcode, 6, Reg::B)),
        0x71 => Some(bit(opcode, 6, Reg::C)),
        0x72 => Some(bit(opcode, 6, Reg::D)),
        0x73 => Some(bit(opcode, 6, Reg::E)),
        0x74 => Some(bit(opcode, 6, Reg::H)),
        0x75 => Some(bit(opcode, 6, Reg::L)),
        0x76 => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(6)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x7F => Some(bit(opcode, 7, Reg::A)),
        0x78 => Some(bit(opcode, 7, Reg::B)),
        0x79 => Some(bit(opcode, 7, Reg::C)),
        0x7A => Some(bit(opcode, 7, Reg::D)),
        0x7B => Some(bit(opcode, 7, Reg::E)),
        0x7C => Some(bit(opcode, 7, Reg::H)),
        0x7D => Some(bit(opcode, 7, Reg::L)),
        0x7E => Some(InstructionBuilder::new(opcode, Mnemonic::BIT, 16).args(
            Operand::Value(ValueType::Constant(7)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        // SET b,r
        // Set bit b in r
        0xC7 => Some(set(opcode, 0, Reg::A)),
        0xC0 => Some(set(opcode, 0, Reg::B)),
        0xC1 => Some(set(opcode, 0, Reg::C)),
        0xC2 => Some(set(opcode, 0, Reg::D)),
        0xC3 => Some(set(opcode, 0, Reg::E)),
        0xC4 => Some(set(opcode, 0, Reg::H)),
        0xC5 => Some(set(opcode, 0, Reg::L)),
        0xC6 => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(0)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xCF => Some(set(opcode, 1, Reg::A)),
        0xC8 => Some(set(opcode, 1, Reg::B)),
        0xC9 => Some(set(opcode, 1, Reg::C)),
        0xCA => Some(set(opcode, 1, Reg::D)),
        0xCB => Some(set(opcode, 1, Reg::E)),
        0xCC => Some(set(opcode, 1, Reg::H)),
        0xCD => Some(set(opcode, 1, Reg::L)),
        0xCE => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(1)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xD7 => Some(set(opcode, 2, Reg::A)),
        0xD0 => Some(set(opcode, 2, Reg::B)),
        0xD1 => Some(set(opcode, 2, Reg::C)),
        0xD2 => Some(set(opcode, 2, Reg::D)),
        0xD3 => Some(set(opcode, 2, Reg::E)),
        0xD4 => Some(set(opcode, 2, Reg::H)),
        0xD5 => Some(set(opcode, 2, Reg::L)),
        0xD6 => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(2)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xDF => Some(set(opcode, 3, Reg::A)),
        0xD8 => Some(set(opcode, 3, Reg::B)),
        0xD9 => Some(set(opcode, 3, Reg::C)),
        0xDA => Some(set(opcode, 3, Reg::D)),
        0xDB => Some(set(opcode, 3, Reg::E)),
        0xDC => Some(set(opcode, 3, Reg::H)),
        0xDD => Some(set(opcode, 3, Reg::L)),
        0xDE => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(3)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xE7 => Some(set(opcode, 4, Reg::A)),
        0xE0 => Some(set(opcode, 4, Reg::B)),
        0xE1 => Some(set(opcode, 4, Reg::C)),
        0xE2 => Some(set(opcode, 4, Reg::D)),
        0xE3 => Some(set(opcode, 4, Reg::E)),
        0xE4 => Some(set(opcode, 4, Reg::H)),
        0xE5 => Some(set(opcode, 4, Reg::L)),
        0xE6 => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(4)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xEF => Some(set(opcode, 5, Reg::A)),
        0xE8 => Some(set(opcode, 5, Reg::B)),
        0xE9 => Some(set(opcode, 5, Reg::C)),
        0xEA => Some(set(opcode, 5, Reg::D)),
        0xEB => Some(set(opcode, 5, Reg::E)),
        0xEC => Some(set(opcode, 5, Reg::H)),
        0xED => Some(set(opcode, 5, Reg::L)),
        0xEE => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(5)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xF7 => Some(set(opcode, 6, Reg::A)),
        0xF0 => Some(set(opcode, 6, Reg::B)),
        0xF1 => Some(set(opcode, 6, Reg::C)),
        0xF2 => Some(set(opcode, 6, Reg::D)),
        0xF3 => Some(set(opcode, 6, Reg::E)),
        0xF4 => Some(set(opcode, 6, Reg::H)),
        0xF5 => Some(set(opcode, 6, Reg::L)),
        0xF6 => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(6)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xFF => Some(set(opcode, 7, Reg::A)),
        0xF8 => Some(set(opcode, 7, Reg::B)),
        0xF9 => Some(set(opcode, 7, Reg::C)),
        0xFA => Some(set(opcode, 7, Reg::D)),
        0xFB => Some(set(opcode, 7, Reg::E)),
        0xFC => Some(set(opcode, 7, Reg::H)),
        0xFD => Some(set(opcode, 7, Reg::L)),
        0xFE => Some(InstructionBuilder::new(opcode, Mnemonic::SET, 16).args(
            Operand::Value(ValueType::Constant(7)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        // RES b,r
        // Reset bit b in r
        0x87 => Some(res(opcode, 0, Reg::A)),
        0x80 => Some(res(opcode, 0, Reg::B)),
        0x81 => Some(res(opcode, 0, Reg::C)),
        0x82 => Some(res(opcode, 0, Reg::D)),
        0x83 => Some(res(opcode, 0, Reg::E)),
        0x84 => Some(res(opcode, 0, Reg::H)),
        0x85 => Some(res(opcode, 0, Reg::L)),
        0x86 => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(0)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x8F => Some(res(opcode, 1, Reg::A)),
        0x88 => Some(res(opcode, 1, Reg::B)),
        0x89 => Some(res(opcode, 1, Reg::C)),
        0x8A => Some(res(opcode, 1, Reg::D)),
        0x8B => Some(res(opcode, 1, Reg::E)),
        0x8C => Some(res(opcode, 1, Reg::H)),
        0x8D => Some(res(opcode, 1, Reg::L)),
        0x8E => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(1)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x97 => Some(res(opcode, 2, Reg::A)),
        0x90 => Some(res(opcode, 2, Reg::B)),
        0x91 => Some(res(opcode, 2, Reg::C)),
        0x92 => Some(res(opcode, 2, Reg::D)),
        0x93 => Some(res(opcode, 2, Reg::E)),
        0x94 => Some(res(opcode, 2, Reg::H)),
        0x95 => Some(res(opcode, 2, Reg::L)),
        0x96 => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(2)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0x9F => Some(res(opcode, 3, Reg::A)),
        0x98 => Some(res(opcode, 3, Reg::B)),
        0x99 => Some(res(opcode, 3, Reg::C)),
        0x9A => Some(res(opcode, 3, Reg::D)),
        0x9B => Some(res(opcode, 3, Reg::E)),
        0x9C => Some(res(opcode, 3, Reg::H)),
        0x9D => Some(res(opcode, 3, Reg::L)),
        0x9E => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(3)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xA7 => Some(res(opcode, 4, Reg::A)),
        0xA0 => Some(res(opcode, 4, Reg::B)),
        0xA1 => Some(res(opcode, 4, Reg::C)),
        0xA2 => Some(res(opcode, 4, Reg::D)),
        0xA3 => Some(res(opcode, 4, Reg::E)),
        0xA4 => Some(res(opcode, 4, Reg::H)),
        0xA5 => Some(res(opcode, 4, Reg::L)),
        0xA6 => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(4)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xAF => Some(res(opcode, 5, Reg::A)),
        0xA8 => Some(res(opcode, 5, Reg::B)),
        0xA9 => Some(res(opcode, 5, Reg::C)),
        0xAA => Some(res(opcode, 5, Reg::D)),
        0xAB => Some(res(opcode, 5, Reg::E)),
        0xAC => Some(res(opcode, 5, Reg::H)),
        0xAD => Some(res(opcode, 5, Reg::L)),
        0xAE => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(5)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xB7 => Some(res(opcode, 6, Reg::A)),
        0xB0 => Some(res(opcode, 6, Reg::B)),
        0xB1 => Some(res(opcode, 6, Reg::C)),
        0xB2 => Some(res(opcode, 6, Reg::D)),
        0xB3 => Some(res(opcode, 6, Reg::E)),
        0xB4 => Some(res(opcode, 6, Reg::H)),
        0xB5 => Some(res(opcode, 6, Reg::L)),
        0xB6 => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(6)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
        )),

        0xBF => Some(res(opcode, 7, Reg::A)),
        0xB8 => Some(res(opcode, 7, Reg::B)),
        0xB9 => Some(res(opcode, 7, Reg::C)),
        0xBA => Some(res(opcode, 7, Reg::D)),
        0xBB => Some(res(opcode, 7, Reg::E)),
        0xBC => Some(res(opcode, 7, Reg::H)),
        0xBD => Some(res(opcode, 7, Reg::L)),
        0xBE => Some(InstructionBuilder::new(opcode, Mnemonic::RES, 16).args(
            Operand::Value(ValueType::Constant(7)),
            Operand::Reference(Ref::Address(Addr::Register(Reg::HL))),
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
        return Some(ld_rr(opcode, r1, r2));
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

    InstructionBuilder::new(opcode, Mnemonic::LD, cycle_count)
        .args(Operand::Reference(r1), Operand::Value(r2))
}

fn ld_rn(opcode: u8, register: Reg) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 12 } else { 8 };
    let op = if register == Reg::HL {
        Ref::Address(Addr::Register(Reg::HL))
    } else {
        Ref::Register(register)
    };
    InstructionBuilder::new(opcode, Mnemonic::LD, cycle_count)
        .args(Operand::Reference(op), Operand::Value(ValueType::Immediate))
}

fn ld_rn16(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::LD, 16).args(
        Operand::Reference(Ref::Register(register)),
        Operand::Value(ValueType::Address(Addr::Immediate)),
    )
}

fn ld_n16r(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::LD, 16).args(
        Operand::Reference(Ref::Address(Addr::Immediate)),
        Operand::Value(ValueType::Register(register)),
    )
}

fn ld_r16n16(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::LD, 12).args(
        Operand::Reference(Ref::Register(register)),
        Operand::Value(ValueType::Immediate16),
    )
}

fn push(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::PUSH, 16)
        .arg(Operand::Value(ValueType::Register(register)))
}

fn pop(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::POP, 12)
        .arg(Operand::Reference(Ref::Register(register)))
}

fn add_an(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::ADD, 4).args(
        Operand::Reference(Ref::Register(Reg::A)),
        Operand::Value(ValueType::Register(register)),
    )
}

fn adc_an(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::ADC)
}

fn sub_an(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::SUB)
}

fn sbc_an(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::SBC)
}

fn and(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::AND)
}

fn or(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::OR)
}

fn xor(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::XOR)
}

fn cp(opcode: u8, register: Reg) -> InstructionInfo {
    alu(opcode, register, Mnemonic::CP)
}

fn inc(opcode: u8, register: Reg) -> InstructionInfo {
    incdec(opcode, register, Mnemonic::INC)
}

fn dec(opcode: u8, register: Reg) -> InstructionInfo {
    incdec(opcode, register, Mnemonic::DEC)
}

fn incdec(opcode: u8, register: Reg, mnemonic: Mnemonic) -> InstructionInfo {
    let cycle_count = if register.is16bit() { 8 } else { 4 };
    InstructionBuilder::new(opcode, mnemonic, cycle_count)
        .arg(Operand::Reference(Ref::Register(register)))
}

fn alu(opcode: u8, register: Reg, mnemonic: Mnemonic) -> InstructionInfo {
    let cycle_count = if register == Reg::HL { 8 } else { 4 };
    let op = if register == Reg::HL {
        ValueType::Address(Addr::Register(Reg::HL))
    } else {
        ValueType::Register(register)
    };
    InstructionBuilder::new(opcode, mnemonic, cycle_count).arg(Operand::Value(op))
}

fn add_hl(opcode: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::ADD, 8).args(
        Operand::Reference(Ref::Register(Reg::HL)),
        Operand::Value(ValueType::Register(register)),
    )
}

fn rlc(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::RLC)
}

fn rl(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::RL)
}

fn rrc(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::RRC)
}

fn rr(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::RR)
}

fn swap(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::SWAP)
}

fn sla(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::SLA)
}

fn sra(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::SRA)
}

fn srl(opcode: u8, register: Reg) -> InstructionInfo {
    rotates(opcode, register, Mnemonic::SRL)
}

fn bit(opcode: u8, bit: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::BIT, 8).args(
        Operand::Value(ValueType::Constant(u16::from(bit))),
        Operand::Reference(Ref::Register(register)),
    )
}

fn set(opcode: u8, bit: u8, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::SET, 8).args(
        Operand::Value(ValueType::Constant(u16::from(bit))),
        Operand::Reference(Ref::Register(register)),
    )
}

fn res(opcode: u8, bit: u16, register: Reg) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::RES, 8).args(
        Operand::Value(ValueType::Constant(bit as u16)),
        Operand::Reference(Ref::Register(register)),
    )
}

fn rotates(opcode: u8, register: Reg, mnemonic: Mnemonic) -> InstructionInfo {
    InstructionBuilder::new(opcode, mnemonic, 8).arg(Operand::Reference(Ref::Register(register)))
}

fn jp(opcode: u8, condition: Condition) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::JP, 12).args(
        Operand::Condition(condition),
        Operand::Value(ValueType::Immediate16),
    )
}

fn jr(opcode: u8, condition: Condition) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::JR, 8).args(
        Operand::Condition(condition),
        Operand::Value(ValueType::Immediate),
    )
}

fn call(opcode: u8, condition: Condition) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::CALL, 12).args(
        Operand::Condition(condition),
        Operand::Value(ValueType::Immediate16),
    )
}

fn rst(opcode: u8, address: u16) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::RST, 32)
        .arg(Operand::Value(ValueType::Constant(address)))
}

fn ret(opcode: u8, condition: Condition) -> InstructionInfo {
    InstructionBuilder::new(opcode, Mnemonic::RET, 8).arg(Operand::Condition(condition))
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
