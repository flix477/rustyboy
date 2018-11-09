use processor::instruction::*;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8, immediate: u16) -> Option<InstructionInfo> {
        match opcode {
            // NOP
            0 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::NOP,
                None,
                0
            )),
            // LD r -> n
            0x06 => Some(Self::build_ld_r_n(
                opcode,
                Register::B,
                immediate
            )),
            0x0E => Some(Self::build_ld_r_n(
                opcode,
                Register::C,
                immediate
            )),
            0x16 => Some(Self::build_ld_r_n(
                opcode,
                Register::D,
                immediate
            )),
            0x1E => Some(Self::build_ld_r_n(
                opcode,
                Register::E,
                immediate
            )),
            0x26 => Some(Self::build_ld_r_n(
                opcode,
                Register::H,
                immediate
            )),
            0x2E => Some(Self::build_ld_r_n(
                opcode,
                Register::L,
                immediate
            )),
            0x36 => Some(Self::build_ld_r_n(
                opcode,
                Register::HL,
                immediate
            )),
            // HALT
            0x76 => Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::HALT,
                None,
                0 // TODO: how many cycles for HALT?
            )),
            // LD r -> r
            0x40..=0x7F => Self::parse_ld_r_r(opcode),
            0x02 =>
            _ => None
        }
    }

    pub fn parse_ld_r_r(opcode: u8) -> Option<InstructionInfo> {
        let r1 = match opcode {
            0x78..=0x7F => Some(Register::A),
            0x40..=0x46 => Some(Register::B),
            0x48..=0x4E => Some(Register::C),
            0x50..=0x56 => Some(Register::D),
            0x58..=0x5E => Some(Register::E),
            0x60..=0x66 => Some(Register::H),
            0x68..=0x6E => Some(Register::L),
            0x70..=0x75 => Some(Register::HL),
            _ => None
        };

        if r1.is_none() { return None; }
        let r1 = r1.unwrap();

        // get second hex value (f7 -> 7)
        let r2 = match opcode & 0xf {
            0 | 0x8 => Some(Register::B),
            1 | 0x9 => Some(Register::C),
            2 | 0xA => Some(Register::D),
            3 | 0xB => Some(Register::E),
            4 | 0xC => Some(Register::H),
            5 | 0xD => Some(Register::L),
            6 | 0xE => Some(Register::HL),
            0xF if r1 == Register::A =>
                Some(Register::A), // could be reduced to 0xF, not sure
            _ => None
        };

        if let Some(r2) = r2 {
            let mut cycle_count = 4;
            if r1 == Register::HL || r2 == Register::HL {
                cycle_count = 8
            }
            return Some(InstructionInfo::new(
                opcode,
                InstructionMnemonic::LD,
                Some(vec![
                    Operand::Register(r1),
                    Operand::Register(r2),
                ]),
                cycle_count
            ));
        }
        return None;
    }

    pub fn build_ld_r_r(opcode: u8, r1: Register, r2: Register)
        -> InstructionInfo
    {
        let cycle_count = if r1.is16bit() || r2.is16bit() { 8 } else { 4 };
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(r1),
                Operand::Register(r2),
            ]),
            cycle_count
        );
    }

    pub fn build_ld_r_n(opcode: u8, register: Register, immediate: u16)
        -> InstructionInfo
    {
        let cycle_count = if register == Register::HL { 12 } else { 8 };
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(register),
                Operand::Immediate(immediate)
            ]),
            cycle_count
        );
    }

    pub fn build_ld_n_r(opcode: u8, register: Register, immediate: u16)
                        -> InstructionInfo
    {
        let cycle_count = if register == Register::HL { 12 } else { 8 };
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Register(register),
                Operand::Immediate(immediate)
            ]),
            cycle_count
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn coverage() {
        let mut covered = Vec::new();
        for i in 0..0xff {
            if let Some(res) = Decoder::decode_opcode(i, 0) {
                covered.push(i);
            }
        }
        assert_eq!(covered.len(), 0xff);
    }
}