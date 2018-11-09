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