use super::{Command, CommandResult, DebuggerState};
use crate::shell_debugger::pretty_print::format_registers;
use crate::util::parse_register;
use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::processor::registers::RegisterType;
use rustyboy_core::util::parse_hex::parse_hex;

const MATCHING_VALUES: &[&str] = &["status", "s"];

pub enum StatusType {
    Address(u16),
    Immediate,
    Register(RegisterType),
    Registers,
}

impl StatusType {
    pub fn parse(values: &[&str]) -> Option<StatusType> {
        let status = *values.get(0).unwrap_or(&"registers");
        match status {
            "address" | "a" => {
                let line: u16 = parse_hex(values.get(1)?)?;
                Some(StatusType::Address(line))
            }
            "immediate" | "i" => Some(StatusType::Immediate),
            "register" | "r" => {
                let register = values.get(1)?;
                let register = parse_register(register)?;
                Some(StatusType::Register(register))
            }
            "registers" => Some(StatusType::Registers),
            _ => None,
        }
    }
}

pub struct StatusCommand {}

impl StatusCommand {
    pub fn create_command() -> Box<dyn Command> {
        Box::new(StatusCommand {})
    }
}

impl Command for StatusCommand {
    fn matching_value(&self) -> &[&str] {
        MATCHING_VALUES
    }

    fn execute(
        &self,
        input: &[&str],
        _: &mut DebuggerState,
        debug_info: &DebugInfo<'_>,
        bus: &dyn Bus,
    ) -> CommandResult {
        if let Some(status_type) = StatusType::parse(&input[1..]) {
            match status_type {
                StatusType::Address(address) => println!("0x{:X}", bus.read(address)),
                StatusType::Immediate => {
                    let pc = debug_info.registers.reg(RegisterType::PC);
                    println!("0x{:X}", bus.read(pc));
                }
                StatusType::Register(register) => {
                    println!("0x{:X}", debug_info.registers.reg(register))
                }
                StatusType::Registers => println!("{}", format_registers(debug_info.registers)),
            }
        } else {
            println!(
                "Invalid command for status (address [address] | register [register] | registers)"
            )
        }

        CommandResult::Continue
    }
}
