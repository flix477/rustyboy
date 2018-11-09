use processor::instruction::*;

pub struct Decoder;

impl Decoder {
    pub fn decode_opcode(opcode: u8, immediate: u16) -> InstructionInfo {
        match opcode {
            0 => InstructionInfo {
                opcode,
                mnemonic: Instruction::NOP,
                operands: [],
                cycle_count: 0
            },
            // LD r -> n
            0x06 => InstructionInfo::build_ld_r_n(
                opcode,
                Register::B,
                immediate
            ),
            0x0E => InstructionInfo::build_ld_r_n(
                opcode,
                Register::C,
                immediate
            ),
            0x16 => InstructionInfo::build_ld_r_n(
                opcode,
                Register::D,
                immediate
            ),
            0x1E => InstructionInfo::build_ld_r_n(
                opcode,
                Register::E,
                immediate
            ),
            0x26 => InstructionInfo::build_ld_r_n(
                opcode,
                Register::H,
                immediate
            ),
            0x2E => InstructionInfo::build_ld_r_n(
                opcode,
                Register::L,
                immediate
            ),
            // LD r -> r
            0x78..=0x7F => {
                let r1 = Register::A;
                let r2 = match opcode {
                    0x7F => Register::A,
                    0x78 => Register::B,
                    0x79 => Register::C,
                    0x7A => Register::D,
                    0x7B => Register::E,
                    0x7C => Register::H,
                    0x7D => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x7E => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::A,
                Register::HL
            ),
            0x40..=0x45 => {
                let r1 = Register::B;
                let r2 = match opcode {
                    0x40 => Register::B,
                    0x41 => Register::C,
                    0x42 => Register::D,
                    0x43 => Register::E,
                    0x44 => Register::H,
                    0x45 => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x46 => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::B,
                Register::HL
            ),
            0x48..=0x4D => {
                let r1 = Register::C;
                let r2 = match opcode {
                    0x48 => Register::B,
                    0x49 => Register::C,
                    0x4A => Register::D,
                    0x4B => Register::E,
                    0x4C => Register::H,
                    0x4D => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x4E => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::C,
                Register::HL
            ),
            0x50..=0x55 => {
                let r1 = Register::D;
                let r2 = match opcode {
                    0x50 => Register::B,
                    0x51 => Register::C,
                    0x52 => Register::D,
                    0x53 => Register::E,
                    0x54 => Register::H,
                    0x55 => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x56 => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::D,
                Register::HL
            ),
            0x58..=0x5D => {
                let r1 = Register::E;
                let r2 = match opcode {
                    0x58 => Register::B,
                    0x59 => Register::C,
                    0x5A => Register::D,
                    0x5B => Register::E,
                    0x5C => Register::H,
                    0x5D => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x5E => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::E,
                Register::HL
            ),
            0x60..=0x65 => {
                let r1 = Register::H;
                let r2 = match opcode {
                    0x60 => Register::B,
                    0x61 => Register::C,
                    0x62 => Register::D,
                    0x63 => Register::E,
                    0x64 => Register::H,
                    0x65 => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x66 => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::H,
                Register::HL
            ),
            0x68..=0x6D => {
                let r1 = Register::L;
                let r2 = match opcode {
                    0x68 => Register::B,
                    0x69 => Register::C,
                    0x6A => Register::D,
                    0x6B => Register::E,
                    0x6C => Register::H,
                    0x6D => Register::L
                };
                return InstructionInfo::build_ld_r_r(opcode, r1, r2);
            }
            0x6E => InstructionInfo::build_ld_r_r16(
                opcode,
                Register::L,
                Register::HL
            ),
            0x70..=0x75 => {
                let r1 = Register::HL;
                let r2 = match opcode {
                    0x68 => Register::B,
                    0x69 => Register::C,
                    0x6A => Register::D,
                    0x6B => Register::E,
                    0x6C => Register::H,
                    0x6D => Register::L
                };
                return InstructionInfo::build_ld_r_r16(opcode, r1, r2);
            }
        }
    }
}