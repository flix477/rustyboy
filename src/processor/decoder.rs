use processor::instruction::*;
use processor::registers::RegisterType;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8)
        -> Option<InstructionInfo>
    {
        match opcode {
            // NOP
            0 => Some(Self::nop(opcode)),

            // LD n,A
            0x2 => Some(Self::ld_rr(
                opcode,
                RegisterType::BC,
                RegisterType::A
            )),
            0x12 => Some(Self::ld_rr(
                opcode,
                RegisterType::DE,
                RegisterType::A
            )),
            0xEA => Some(Self::ld_n16r(
                opcode,
                RegisterType::A
            )),

            // LD A,n
            // put value n into A
            0x0A => Some(Self::ld_rr(
                opcode,
                RegisterType::A,
                RegisterType::BC
            )),
            0x1A => Some(Self::ld_rr(
                opcode,
                RegisterType::A,
                RegisterType::DE
            )),
            0x7E => Some(Self::ld_rr(
                opcode,
                RegisterType::A,
                RegisterType::HL
            )),
            0xFA => Some(Self::ld_rn16(
                opcode,
                RegisterType::A
            )),
            // TODO: 0x3E

            // LD nn,n
            // put value nn into n
            0x06 => Some(Self::ld_rn(
                opcode,
                RegisterType::B
            )),
            0x0E => Some(Self::ld_rn(
                opcode,
                RegisterType::C
            )),
            0x16 => Some(Self::ld_rn(
                opcode,
                RegisterType::D
            )),
            0x1E => Some(Self::ld_rn(
                opcode,
                RegisterType::E
            )),
            0x26 => Some(Self::ld_rn(
                opcode,
                RegisterType::H
            )),
            0x2E => Some(Self::ld_rn(
                opcode,
                RegisterType::L
            )),
            0x36 => Some(Self::ld_rn(
                opcode,
                RegisterType::HL
            )),

            // HALT
            0x76 => Some(Self::nop(opcode)),

            // LD r1,r2
            // put value r2 in r1
            0x40..=0x7F => Self::parse_ld_rr(opcode),

            // LD A,(C)
            // Put value at address $FF00 + register C into A
            0xF2 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(RegisterType::A),
                    Operand::IncrementedRegister(RegisterType::C)
                ]),
                8
            )),

            // LD (C),A
            // Put A into address $FF00 + register C
            0xE2 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::IncrementedRegister(RegisterType::C),
                    Operand::Register(RegisterType::A)
                ]),
                8
            )),

            // LDD A,(HL)
            0x3A => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDD,
                Some(vec![
                    Operand::Register(RegisterType::A),
                    Operand::Register(RegisterType::HL)
                ]),
                8
            )),

            // LDD (HL),A
            0x32 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDD,
                Some(vec![
                    Operand::Register(RegisterType::HL),
                    Operand::Register(RegisterType::A)
                ]),
                8
            )),

            // LDI A,(HL)
            0x2A => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDI,
                Some(vec![
                    Operand::Register(RegisterType::A),
                    Operand::Register(RegisterType::HL)
                ]),
                8
            )),

            // LDI (HL),A
            0x22 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDI,
                Some(vec![
                    Operand::Register(RegisterType::HL),
                    Operand::Register(RegisterType::A)
                ]),
                8
            )),

            // LDH (n),A
            0xE0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::IncrementedImmediate,
                    Operand::Register(RegisterType::A)
                ]),
                12
            )),

            // LDH A,(n)
            0xF0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(RegisterType::A),
                    Operand::IncrementedImmediate
                ]),
                12
            )),

            // LD n,nn 16bit
            0x01 => Some(Self::ld_r16n16(opcode, RegisterType::BC)),
            0x11 => Some(Self::ld_r16n16(opcode, RegisterType::DE)),
            0x21 => Some(Self::ld_r16n16(opcode, RegisterType::HL)),
            0x31 => Some(Self::ld_r16n16(opcode, RegisterType::SP)),

            // LD SP,HL
            0xF9 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(RegisterType::SP),
                    Operand::Register(RegisterType::HL)
                ]),
                8
            )),

            // LDHL SP,n
            0xF8 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LDHL,
                Some(vec![
                    Operand::Register(RegisterType::SP),
                    Operand::Immediate
                ]),
                12
            )),

            // LD (nn),SP
            0x08 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Immediate16,
                    Operand::Register(RegisterType::SP)
                ]),
                20
            )),

            // PUSH nn
            0xF5 => Some(Self::push(opcode, RegisterType::AF)),
            0xC5 => Some(Self::push(opcode, RegisterType::BC)),
            0xD5 => Some(Self::push(opcode, RegisterType::DE)),
            0xE5 => Some(Self::push(opcode, RegisterType::HL)),

            // POP nn
            0xF1 => Some(Self::push(opcode, RegisterType::AF)),
            0xC1 => Some(Self::push(opcode, RegisterType::BC)),
            0xD1 => Some(Self::push(opcode, RegisterType::DE)),
            0xE1 => Some(Self::push(opcode, RegisterType::HL)),

            // ADD A,n
            // add n to A
            0x87 => Some(Self::add_an(opcode, RegisterType::A)),
            0x80 => Some(Self::add_an(opcode, RegisterType::B)),
            0x81 => Some(Self::add_an(opcode, RegisterType::C)),
            0x82 => Some(Self::add_an(opcode, RegisterType::D)),
            0x83 => Some(Self::add_an(opcode, RegisterType::E)),
            0x84 => Some(Self::add_an(opcode, RegisterType::H)),
            0x85 => Some(Self::add_an(opcode, RegisterType::L)),
            0x86 => Some(Self::add_an(opcode, RegisterType::HL)),
            0xC6 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::ADD,
                Some(vec![
                    Operand::Register(RegisterType::A),
                    Operand::Immediate16
                ]),
                8
            )),

            // ADC A,n
            // Add n + Carry flag to A
            0x8F => Some(Self::adc_an(opcode, RegisterType::A)),
            0x89 => Some(Self::adc_an(opcode, RegisterType::B)),
            0x8A => Some(Self::adc_an(opcode, RegisterType::C)),
            0x8B => Some(Self::adc_an(opcode, RegisterType::D)),
            0x8C => Some(Self::adc_an(opcode, RegisterType::E)),
            0x8D => Some(Self::adc_an(opcode, RegisterType::H)),
            0x8E => Some(Self::adc_an(opcode, RegisterType::L)),
            0x8F => Some(Self::adc_an(opcode, RegisterType::HL)),
            0xCE => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::ADC,
                Some(vec![
                    Operand::Register(RegisterType::A),
                    Operand::Immediate16
                ]),
                8
            )),

            _ => None
        }
    }

    fn parse_ld_rr(opcode: u8) -> Option<InstructionInfo> {
        let r1 = match opcode {
            0x78..=0x7F => Some(RegisterType::A),
            0x40..=0x46 => Some(RegisterType::B),
            0x48..=0x4F => Some(RegisterType::C),
            0x50..=0x56 => Some(RegisterType::D),
            0x58..=0x5F => Some(RegisterType::E),
            0x60..=0x66 => Some(RegisterType::H),
            0x68..=0x6F => Some(RegisterType::L),
            0x70..=0x77 => Some(RegisterType::HL),
            _ => None
        };

        if r1.is_none() { return None; }
        let r1 = r1.unwrap();

        // get second hex value (f7 -> 7)
        let r2 = match opcode & 0xf {
            0 | 0x8 => Some(RegisterType::B),
            1 | 0x9 => Some(RegisterType::C),
            2 | 0xA => Some(RegisterType::D),
            3 | 0xB => Some(RegisterType::E),
            4 | 0xC => Some(RegisterType::H),
            5 | 0xD => Some(RegisterType::L),
            6 | 0xE => Some(RegisterType::HL),
            7 | 0xF => Some(RegisterType::A),
            _ => None
        };

        if let Some(r2) = r2 {
            return Some(Self::ld_rr(opcode, r1, r2));
        }
        return None;
    }

    fn ld_nr(opcode: u8, register: RegisterType)
         -> InstructionInfo
    {
        let cycle_count = if register == RegisterType::HL { 12 } else { 8 };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Immediate,
                Operand::Register(register)
            ]),
            cycle_count
        )
    }

    fn ld_rn(opcode: u8, register: RegisterType)
        -> InstructionInfo
    {
        let cycle_count = if register == RegisterType::HL { 12 } else { 8 };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(register),
                Operand::Immediate
            ]),
            cycle_count
        )
    }

    fn ld_rr(opcode: u8, r1: RegisterType, r2: RegisterType)
         -> InstructionInfo
    {
        let cycle_count = if r1.is16bit() || r2.is16bit() { 8 } else { 4 };
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![Operand::Register(r1), Operand::Register(r2)]),
            cycle_count
        )
    }

    fn ld_rn16(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![Operand::Register(register), Operand::Immediate16]),
            16
        )
    }

    fn ld_n16r(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![Operand::Immediate16, Operand::Register(register)]),
            16
        )
    }

    fn ld_r16n16(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(register),
                Operand::Immediate16
            ]),
            12
        )
    }

    fn push(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::PUSH,
            Some(vec![
                Operand::Register(register)
            ]),
            16
        )
    }

    fn pop(opcode: u8, register: RegisterType) -> InstructionInfo {
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

    fn add_an(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::ADD,
            Some(vec![
                Operand::Register(RegisterType::A),
                Operand::Register(register)
            ]),
            if register == RegisterType::HL { 8 } else { 4 }
        )
    }

    fn adc_an(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::ADC,
            Some(vec![
                Operand::Register(RegisterType::A),
                Operand::Register(register)
            ]),
            if register == RegisterType::HL { 8 } else { 4 }
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