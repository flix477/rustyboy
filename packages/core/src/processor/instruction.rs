use crate::processor::registers::flag_register::Flag;
use crate::processor::registers::RegisterType;

/// Represents a GameBoy instruction prefix.
/// The GameBoy has a special opcode CB that acts as a prefix to enable having more instructions
/// without overlapping with the existing ones.
#[derive(Copy, Clone)]
pub enum Prefix {
    CB,
    None,
}

/// Represents the mnemonic of a GameBoy instruction.
/// The mnemonic is simply the name of the instruction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mnemonic {
    CB,
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
    AND,
    XOR,
    OR,
    CP,
    INC,
    DEC,
    DAA,
    CPL,
    RLC,
    RLCA,
    RL,
    RLA,
    RRC,
    RRCA,
    RR,
    RRA,
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
    RST,
}

/// Represents a condition operand.
/// The first value is the CPU flag to check and the second value is the expected value.
/// If the flag's value is equal to the expected value, the condition is fulfilled.
#[derive(Copy, Clone, Debug)]
pub struct Condition(pub Flag, pub bool);

impl ToString for Condition {
    fn to_string(&self) -> String {
        format!("if {}{}", if self.1 { "" } else { "!" }, self.0.to_string())
    }
}

/// Represents a GameBoy instruction operand.
/// An operand can simply be seen as an argument given to a function.
/// GameBoy instructions typically have 0 to 2 operands.
#[derive(Copy, Clone, Debug)]
pub enum Operand {
    /// An operand that represents a reference to mutate, like a register or an address
    Reference(Reference),
    /// An operand that represents an integer value
    Value(ValueType),
    /// An operand that represents a condition to fulfill
    Condition(Condition),
}

/// An operand type that represents a reference to mutate in an instruction
#[derive(Copy, Clone, Debug)]
pub enum Reference {
    /// Represents a register to mutate
    Register(RegisterType),
    /// Represents a bus address to mutate
    Address(AddressType),
}

impl ToString for Reference {
    fn to_string(&self) -> String {
        match *self {
            Reference::Register(register) => register.to_string(),
            Reference::Address(address) => address.to_string(),
        }
    }
}

impl Reference {
    /// Returns true if the given reference is a 16-bit reference and false if it is an 8-bit one.
    /// Only 16-bit registers (BC, DE, HL, etc) return true to this.
    pub fn is16bit(self) -> bool {
        if let Reference::Register(register) = self {
            register.is16bit()
        } else {
            false
        }
    }
}

/// Represents the type of an address used as an operand.
/// All addresses are 16-bit.
#[derive(Copy, Clone, Debug)]
pub enum AddressType {
    /// Represents an address which is the value of a register
    Register(RegisterType),
    /// Represents an address which is the value of a register, incremented with 0xFF00
    IncRegister(RegisterType),
    /// Represents an address which is the value next in the program counter
    Immediate,
    /// Represents an address which is the value next in the program counter,
    /// incremented with 0xFF00
    IncImmediate,
}

impl ToString for AddressType {
    fn to_string(&self) -> String {
        match *self {
            AddressType::Register(register) => format!("({})", register.to_string()),
            AddressType::IncRegister(register) => format!("({} + 0xFF00)", register.to_string()),
            AddressType::Immediate => "(nn)".to_string(),
            AddressType::IncImmediate => "(nn + 0xFF00)".to_string(),
        }
    }
}

/// Represents the type of a value used as an operand
#[derive(Copy, Clone, Debug)]
pub enum ValueType {
    /// Represents a value in a register
    Register(RegisterType),
    /// Represents an unsigned value which is next in the program counter
    Immediate,
    /// Represents an signed value which is next in the program counter
    SignedImmediate,
    /// Represents an unsigned 16-bit value which is next in the program counter
    Immediate16,
    /// Represents a value stored at an address
    Address(AddressType),
    /// Represents a constant value
    Constant(u16),
}

impl ToString for ValueType {
    fn to_string(&self) -> String {
        match *self {
            ValueType::Register(register) => register.to_string(),
            ValueType::Immediate => "n".to_string(),
            ValueType::SignedImmediate => "±n".to_string(),
            ValueType::Immediate16 => "nn".to_string(),
            ValueType::Address(address) => address.to_string(),
            ValueType::Constant(value) => value.to_string(),
        }
    }
}

/// Contains information about a GameBoy instruction
#[derive(Debug, Clone)]
pub struct InstructionInfo {
    opcode: u8,
    mnemonic: Mnemonic,
    operands: Option<Vec<Operand>>,
    cycle_count: u8,
}

impl InstructionInfo {
    pub fn new(
        opcode: u8,
        mnemonic: Mnemonic,
        operands: Option<Vec<Operand>>,
        cycle_count: u8,
    ) -> InstructionInfo {
        InstructionInfo {
            opcode,
            mnemonic,
            operands,
            cycle_count,
        }
    }

    pub fn mnemonic(&self) -> &Mnemonic {
        &self.mnemonic
    }

    // TODO: why is this not just a Vec
    pub fn operands(&self) -> &Option<Vec<Operand>> {
        &self.operands
    }

    pub fn cycle_count(&self) -> u8 {
        self.cycle_count
    }
}
