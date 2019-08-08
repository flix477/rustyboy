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

/// Represents a condition operand.
/// The first value is the CPU flag to check and the second value is the expected value.
/// If the flag's value is equal to the expected value, the condition is fulfilled.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Condition(pub Flag, pub bool);

impl ToString for Condition {
    fn to_string(&self) -> String {
        format!("if {}{}", if self.1 { "" } else { "!" }, self.0.to_string())
    }
}

/// An operand type that represents a reference to mutate in an instruction
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InstructionInfo {
    pub mnemonic: Mnemonic,
    pub cycle_count: u8
}

impl InstructionInfo {
    pub fn new(mnemonic: Mnemonic, cycle_count: u8) -> Self {
        Self {
            mnemonic,
            cycle_count
        }
    }
}

/// Represents the mnemonic of a GameBoy instruction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mnemonic {
    LD(Reference, ValueType),
    LDD(Reference, ValueType),
    LDI(Reference, ValueType),
    LDHL,
    PUSH(ValueType),
    POP(RegisterType),
    ADD(RegisterType, ValueType),
    ADC(ValueType),
    SUB(ValueType),
    SBC(ValueType),
    AND(ValueType),
    OR(ValueType),
    XOR(ValueType),
    CP(ValueType),
    INC(Reference),
    DEC(Reference),
    DAA,
    CPL,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    RLC(Reference),
    RL(Reference),
    RRC(Reference),
    RR(Reference),
    SWAP(Reference),
    RLCA,
    RLA,
    RRCA,
    RRA,
    SLA(Reference),
    SRA(Reference),
    SRL(Reference),
    BIT(u16, Reference),
    SET(u16, Reference),
    RES(u16, Reference),
    JP(Option<Condition>, ValueType),
    JR(Option<Condition>, ValueType),
    CALL(Option<Condition>, ValueType),
    RST(u16),
    RET(Option<Condition>),
    RETI,
    CB
}

impl Mnemonic {
    pub fn operands(self) -> Vec<Operand> {
        match self {
            Mnemonic::LD(reference, value) => vec![Operand::Reference(reference), Operand::Value(value)],
            Mnemonic::LDD(reference, value) => vec![Operand::Reference(reference), Operand::Value(value)],
            Mnemonic::LDI(reference, value) => vec![Operand::Reference(reference), Operand::Value(value)],
            Mnemonic::PUSH(value) => vec![Operand::Value(value)],
            Mnemonic::POP(register) => vec![Operand::Reference(Reference::Register(register))],
            Mnemonic::ADD(register, value) => vec![Operand::Reference(Reference::Register(register)), Operand::Value(value)],
            Mnemonic::ADC(value) => vec![Operand::Value(value)],
            Mnemonic::SUB(value) => vec![Operand::Value(value)],
            Mnemonic::SBC(value) => vec![Operand::Value(value)],
            Mnemonic::AND(value) => vec![Operand::Value(value)],
            Mnemonic::OR(value) => vec![Operand::Value(value)],
            Mnemonic::XOR(value) => vec![Operand::Value(value)],
            Mnemonic::CP(value) => vec![Operand::Value(value)],
            Mnemonic::INC(reference) => vec![Operand::Reference(reference)],
            Mnemonic::DEC(reference) => vec![Operand::Reference(reference)],
            Mnemonic::RLC(reference) => vec![Operand::Reference(reference)],
            Mnemonic::RL(reference) => vec![Operand::Reference(reference)],
            Mnemonic::RRC(reference) => vec![Operand::Reference(reference)],
            Mnemonic::RR(reference) => vec![Operand::Reference(reference)],
            Mnemonic::SWAP(reference) => vec![Operand::Reference(reference)],
            Mnemonic::SLA(reference) => vec![Operand::Reference(reference)],
            Mnemonic::SRA(reference) => vec![Operand::Reference(reference)],
            Mnemonic::SRL(reference) => vec![Operand::Reference(reference)],
            Mnemonic::BIT(value, reference) => vec![Operand::Value(ValueType::Constant(value)), Operand::Reference(reference)],
            Mnemonic::SET(value, reference) => vec![Operand::Value(ValueType::Constant(value)), Operand::Reference(reference)],
            Mnemonic::RES(value, reference) => vec![Operand::Value(ValueType::Constant(value)), Operand::Reference(reference)],
            Mnemonic::JP(None, value) => vec![Operand::Value(value)],
            Mnemonic::JP(Some(condition), value) => vec![Operand::Condition(condition), Operand::Value(value)],
            Mnemonic::JR(None, value) => vec![Operand::Value(value)],
            Mnemonic::JR(Some(condition), value) => vec![Operand::Condition(condition), Operand::Value(value)],
            Mnemonic::CALL(None, value) => vec![Operand::Value(value)],
            Mnemonic::CALL(Some(condition), value) => vec![Operand::Condition(condition), Operand::Value(value)],
            Mnemonic::RST(value) => vec![Operand::Value(ValueType::Constant(value))],
            Mnemonic::RET(Some(condition)) => vec![Operand::Condition(condition)],
            _ => vec![]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operand {
    Reference(Reference),
    Value(ValueType),
    Condition(Condition)
}