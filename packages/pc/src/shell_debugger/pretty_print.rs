use console::style;

use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::processor::instruction::{AddressType, Reference, ValueType};
use rustyboy_core::processor::instruction::Operand;
use rustyboy_core::processor::registers::RegisterType;

const IMMEDIATE: &'static str = "n";
const IMMEDIATE_16: &'static str = "nn";

pub fn format_debug_info(debug_info: &DebugInfo) -> String {
    let operands = if let Some(operands) = debug_info.instruction.operands() {
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

    let line = format!("0x{:X}", debug_info.line);
    let mnemonic = format!("{:?}", debug_info.instruction.mnemonic());

    format!(
        "{}: {} {}",
        style(line).bold(),
        style(mnemonic).blue(),
        operands
    )
}

fn parse_operand(operand: &Operand) -> String {
    match operand {
        Operand::Reference(reference) => match reference {
            Reference::Register(register) => style_register(*register),
            Reference::Address(address) => parse_address(*address),
        },
        Operand::Value(value) => match value {
            ValueType::Register(register) => style_register(*register),
            ValueType::Address(address) => parse_address(*address),
            ValueType::Constant(constant) => format!("0x{:X}", constant),
            ValueType::Immediate | ValueType::SignedImmediate => IMMEDIATE.to_string(),
            ValueType::Immediate16 => IMMEDIATE_16.to_string(),
        },
        _ => format!("{:?}", operand),
    }
}

fn style_register(register: RegisterType) -> String {
    style(register.to_string()).yellow().to_string()
}

fn parse_address(address: AddressType) -> String {
    let address = match address {
        AddressType::Register(register) => register.to_string(),
        AddressType::IncRegister(register) => format_increment(&register.to_string()),
        AddressType::Immediate => IMMEDIATE_16.to_string(),
        AddressType::IncImmediate => format_increment(IMMEDIATE),
    };

    style(format_address(&address)).magenta().to_string()
}

fn format_address(value: &str) -> String {
    format!("({})", value)
}

fn format_increment(value: &str) -> String {
    format!("FF00+{}", value)
}