use console::style;

use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::debugger::processor_debug_info::ParsedOperand;
use rustyboy_core::processor::instruction::{AddressType, Reference, ValueType};
use rustyboy_core::processor::registers::{RegisterType, Registers};

const IMMEDIATE: &str = "n";
const IMMEDIATE_16: &str = "nn";

pub fn format_registers(registers: &Registers) -> String {
    format!(
        "AF: 0x{:X}\nBC: 0x{:X}\nDE: 0x{:X}\nHL: 0x{:X}\nSP: 0x{:X}\nPC: 0x{:X}",
        registers.reg(RegisterType::AF),
        registers.reg(RegisterType::BC),
        registers.reg(RegisterType::DE),
        registers.reg(RegisterType::HL),
        registers.reg(RegisterType::SP),
        registers.reg(RegisterType::PC)
    )
}

pub fn format_debug_info(debug_info: &DebugInfo) -> String {
    let line = debug_info.cpu_debug_info.current_line();
    let instruction = debug_info.cpu_debug_info.parse_instruction(line);

    let line = format!("0x{:X}", line);

    if let Some(instruction) = instruction {
        let operands = if !instruction.parsed_operands.is_empty() {
            instruction
                .parsed_operands
                .iter()
                .map(|x| parse_operand(*x))
                .enumerate()
                .fold(String::new(), |acc, (index, x)| {
                    if index == 0 {
                        x
                    } else {
                        format!("{}, {}", acc, x)
                    }
                })
        } else {
            String::new()
        };

        let mnemonic = format!("{:?}", instruction.instruction.mnemonic);

        format!(
            "{}: {} {}",
            style(line).bold(),
            style(mnemonic).blue(),
            operands
        )
    } else {
        format!("{}: No instruction", style(line).bold())
    }
}

fn parse_operand(operand: ParsedOperand) -> String {
    match operand {
        ParsedOperand::Reference(reference) => match reference {
            (Reference::Register(register), _) => style_register(register),
            (Reference::Address(address), _) => parse_address(address),
        },
        ParsedOperand::Value((value, parsed_value)) => match value {
            ValueType::Register(register) => style_register(register),
            ValueType::Address(address) => parse_address(address),
            ValueType::Constant(constant) => format!("0x{:X}", constant),
            ValueType::Immediate | ValueType::SignedImmediate => format!("0x{:X}", parsed_value),
            ValueType::Immediate16 => format!("0x{:X}", parsed_value),
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
