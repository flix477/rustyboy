pub enum Instruction {
    LD,
    LDI,
    LDD,
    PUSH,
    POP,
    ADD,
    ADC,
    SUB,
    SBC,
    XOR,
    OR,
    CP,
    INC,
    DEC,
    DAA,
    CPL,
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC,
    RL,
    RR,
    SLA,
    SWAP,
    SRA,
    SRL,
    BIT,
    SET,
    RES,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    JP,
    JR,
    CALL,
    RET,
    RETI,
    RST
}

pub struct InstructionInfo {
    opcode: u8,
    mnemonic: Instruction,
    operands: [Option<Operand>; 2],
    cycle_count: u8
}

impl InstructionInfo {
    pub fn build_ld_r_n(opcode: u8, register: Register, immediate: u16)
        -> InstructionInfo
    {
        return InstructionInfo {
            opcode,
            mnemonic: Instruction::LD,
            operands: [
                Some(Operand::Register(register)),
                (Operand::Immediate(immediate))
            ],
            cycle_count: 8
        }
    }

    pub fn build_ld_r_r(opcode: u8, r1: Register, r2: Register)
         -> InstructionInfo
    {
        return InstructionInfo {
            opcode,
            mnemonic: Instruction::LD,
            operands: [
                Some(Operand::Register(r1)),
                Some(Operand::Register(r2)),
            ],
            cycle_count: 4
        }
    }

    pub fn build_ld_r_r16(opcode: u8, r1: Register, r2: Register)
        -> InstructionInfo
    {
        return InstructionInfo {
            opcode,
            mnemonic: Instruction::LD,
            operands: [
                Some(Operand::Register(r1)),
                Some(Operand::Register(r2)),
            ],
            cycle_count: 8
        }
    }
}

pub enum Operand {
    Register(Register),
    Immediate(u16)
}

pub enum Register {
    AF,
    A,
    F,
    BC,
    B,
    C,
    DE,
    D,
    E,
    HL,
    H,
    L,
    SP,
    PC
}