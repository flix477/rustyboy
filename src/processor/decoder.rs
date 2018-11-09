use processor::instruction::*;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8, immediate: u16) -> Option<InstructionInfo> {
        match opcode {
            // NOP
            0 => Some(InstructionInfo::nop(opcode)),
            // LD A -> n
            0x2 => Some(InstructionInfo::ld_rr(
                opcode,
                Register::BC,
                Register::A
            )),
            0x12 => Some(InstructionInfo::ld_rr(
                opcode,
                Register::DE,
                Register::A
            )),
            0x77 => Some(InstructionInfo::ld_rr(
                opcode,
                Register::HL,
                Register::A
            )),
            // LD n -> A
            0x0A => Some(InstructionInfo::ld_rr(
                opcode,
                Register::A,
                Register::BC
            )),
            0x1A => Some(InstructionInfo::ld_rr(
                opcode,
                Register::DE,
                Register::A
            )),
            0x7E => Some(InstructionInfo::ld_rr(
                opcode,
                Register::HL,
                Register::A
            )),
            // LD A ->
            // LD r -> n
            0x06 => Some(InstructionInfo::ld_rn(
                opcode,
                Register::B,
                immediate
            )),
            0x0E => Some(InstructionInfo::ld_rn(
                opcode,
                Register::C,
                immediate
            )),
            0x16 => Some(InstructionInfo::ld_rn(
                opcode,
                Register::D,
                immediate
            )),
            0x1E => Some(InstructionInfo::ld_rn(
                opcode,
                Register::E,
                immediate
            )),
            0x26 => Some(InstructionInfo::ld_rn(
                opcode,
                Register::H,
                immediate
            )),
            0x2E => Some(InstructionInfo::ld_rn(
                opcode,
                Register::L,
                immediate
            )),
            0x36 => Some(InstructionInfo::ld_rn(
                opcode,
                Register::HL,
                immediate
            )),
            // HALT
            0x76 => Some(InstructionInfo::nop(opcode)),
            // LD r -> r
            0x40..=0x7F => Self::parse_ld_rr(opcode),
            _ => None
        }
    }

    pub fn parse_ld_rr(opcode: u8) -> Option<InstructionInfo> {
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
            return Some(InstructionInfo::ld_rr(opcode, r1, r2));
        }
        return None;
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
        assert_eq!(covered.len(), 0xff - 11);
    }
}