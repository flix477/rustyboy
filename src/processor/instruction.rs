use processor::registers::RegisterType;

#[derive(Copy, Clone)]
pub enum InstructionMnemonic {
    LD,
    LDHL,
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

#[derive(Copy, Clone)]
// Increment versions are incremented with 0xFF00
pub enum Operand {
    Register(RegisterType),
    Immediate,
    Immediate16,
    IncrementedRegister(RegisterType),
    IncrementedImmediate
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

    pub fn mnemonic(&self) -> &InstructionMnemonic {
        &self.mnemonic
    }

    pub fn operands(&self) -> &Option<Vec<Operand>> {
        &self.operands
    }
}
