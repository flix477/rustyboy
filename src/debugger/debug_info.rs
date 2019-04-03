use crate::processor::instruction::{AddressType, Reference, ValueType};
use crate::processor::instruction::{InstructionInfo, Operand};
use crate::processor::registers::{RegisterType, Registers};
use std::fmt::{Debug, Error, Formatter};

const IMMEDIATE: &'static str = "n";
const IMMEDIATE_16: &'static str = "nn";

pub struct DebugInfo<'a> {
    pub registers: &'a Registers,
    pub line: u16,
    pub instruction: &'a InstructionInfo,
}

impl<'a> Debug for DebugInfo<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let operands = if let Some(operands) = self.instruction.operands() {
            operands.iter().map(parse_operand).enumerate().fold(
                String::new(),
                |acc, (idx, operand)| {
                    if idx == 0 {
                        operand
                    } else {
                        format!("{}, {}", acc, operand)
                    }
                },
            )
        } else {
            String::new()
        };

        write!(
            f,
            "0x{:X}: {:?} {}",
            self.line,
            self.instruction.mnemonic(),
            operands
        )
    }
}

fn parse_operand(operand: &Operand) -> String {
    match operand {
        Operand::Reference(reference) => match reference {
            Reference::Register(register) => register.to_string(),
            Reference::Address(address) => parse_address(*address),
        },
        Operand::Value(value) => match value {
            ValueType::Register(register) => register.to_string(),
            ValueType::Address(address) => parse_address(*address),
            ValueType::Constant(constant) => format!("0x{:X}", constant),
            ValueType::Immediate => IMMEDIATE.to_string(),
            ValueType::Immediate16 => IMMEDIATE_16.to_string(),
        },
        _ => format!("{:?}", operand),
    }
}

fn parse_address(address: AddressType) -> String {
    let address = match address {
        AddressType::Register(register) => register.to_string(),
        AddressType::IncRegister(register) => format_increment(&register.to_string()),
        AddressType::Immediate => IMMEDIATE_16.to_string(),
        AddressType::IncImmediate => format_increment(IMMEDIATE),
    };
    format_address(&address)
}

fn format_address(value: &str) -> String {
    format!("({})", value)
}

fn format_increment(value: &str) -> String {
    format!("FF00+{}", value)
}
