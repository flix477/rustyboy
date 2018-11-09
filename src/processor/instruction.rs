pub enum InstructionMnemonic {
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

pub enum Operand {
    Register(Register),
    Immediate(u16)
}

#[derive(PartialEq)]
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

impl Register {
    pub fn is16bit(&self) -> bool {
        return
            self == Register::AF ||
            self == Register::BC ||
            self == Register::DE ||
            self == Register::HL ||
            self == Register::SP ||
            self == Register::PC
        ;
    }
}

pub struct InstructionInfo {
    opcode: u8,
    mnemonic: InstructionMnemonic,
    operands: Option<Vec<Operand>>,
    cycle_count: u8
}

impl InstructionInfo {
    pub fn new(
        opcode: u8,
        mnemonic: InstructionMnemonic,
        operands: Option<Vec<Operand>>,
        cycle_count: u8
    ) -> InstructionInfo {
        return InstructionInfo {
            opcode,
            mnemonic,
            operands,
            cycle_count
        };
    }

    pub fn ld_nr(opcode: u8, immediate: u16, register: Register) -> InstructionInfo {
        let cycle_count = if register == Register::HL { 12 } else { 8 };
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![
                Operand::Immediate(immediate),
                Operand::Register(register)
            ]),
            cycle_count
        );
    }

    pub fn ld_rn(opcode: u8, register: Register, immediate: u16) -> InstructionInfo {
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

    pub fn ld_rr(opcode: u8, r1: Register, r2: Register) -> InstructionInfo {
        let cycle_count = if r1.is16bit() || r2.is16bit() { 8 } else { 4 };
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::LD,
            Some(vec![Operand::Register(r1), Operand::Register(r2)]),
            cycle_count
        );
    }

    pub fn nop(opcode: u8) -> InstructionInfo {
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::NOP,
            None,
            0
        );
    }

    pub fn halt(opcode: u8) -> InstructionInfo {
        return InstructionInfo::new(
            opcode,
            InstructionMnemonic::HALT,
            None,
            0 // TODO: how many cycles for HALT?
        );
    }
}
