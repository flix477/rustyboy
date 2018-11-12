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

            // LD A -> n
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
            0x77 => Some(Self::ld_rr(
                opcode,
                RegisterType::HL,
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

            _ => None
        }
    }

    pub fn parse_ld_rr(opcode: u8) -> Option<InstructionInfo> {
        let r1 = match opcode {
            0x78..=0x7F => Some(RegisterType::A),
            0x40..=0x46 => Some(RegisterType::B),
            0x48..=0x4E => Some(RegisterType::C),
            0x50..=0x56 => Some(RegisterType::D),
            0x58..=0x5E => Some(RegisterType::E),
            0x60..=0x66 => Some(RegisterType::H),
            0x68..=0x6E => Some(RegisterType::L),
            0x70..=0x75 => Some(RegisterType::HL),
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
            0xF if r1 == RegisterType::A =>
                Some(RegisterType::A),
            _ => None
        };

        if let Some(r2) = r2 {
            return Some(Self::ld_rr(opcode, r1, r2));
        }
        return None;
    }

    pub fn ld_nr(opcode: u8, immediate: u16, register: RegisterType)
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

    pub fn ld_rn(opcode: u8, register: RegisterType)
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

    pub fn ld_rr(opcode: u8, r1: RegisterType, r2: RegisterType)
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

    pub fn ld_rn16(opcode: u8, register: RegisterType) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![Operand::Register(register), Operand::Immediate16]),
            16
        )
    }

    pub fn nop(opcode: u8) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::NOP,
            None,
            0
        )
    }

    pub fn halt(opcode: u8) -> InstructionInfo {
        InstructionInfo::new(
            opcode,
            InstructionMnemonic::HALT,
            None,
            0 // TODO: how many cycles for HALT?
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