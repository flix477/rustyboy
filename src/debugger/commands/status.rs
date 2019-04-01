use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult, Debugger};
use crate::debugger::{DebugInfo, DebuggerState};
use crate::processor::lr35902::LR35902;
use crate::processor::registers::{RegisterType, Registers};
use crate::processor::Processor;

const MATCHING_VALUES: &'static [&'static str] = &["status", "s"];

pub enum StatusType {
    Address(u16),
    Register(RegisterType),
    Registers,
}

impl StatusType {
    pub fn parse(values: &[&str]) -> Option<StatusType> {
        let status = *values.get(0)?;
        match status {
            "register" | "r" => {
                let register = values.get(1)?;
                let register = StatusType::parse_register(register)?;
                Some(StatusType::Register(register))
            }
            "address" | "a" => None,
            "registers" => Some(StatusType::Registers),
            _ => None,
        }
    }

    fn parse_register(value: &str) -> Option<RegisterType> {
        match value {
            "a" => Some(RegisterType::A),
            "f" => Some(RegisterType::F),
            "af" => Some(RegisterType::AF),
            "b" => Some(RegisterType::B),
            "c" => Some(RegisterType::C),
            "bc" => Some(RegisterType::BC),
            "d" => Some(RegisterType::D),
            "e" => Some(RegisterType::E),
            "de" => Some(RegisterType::DE),
            "h" => Some(RegisterType::H),
            "l" => Some(RegisterType::L),
            "hl" => Some(RegisterType::HL),
            "sp" => Some(RegisterType::SP),
            "pc" => Some(RegisterType::PC),
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
        debug_info: &DebugInfo,
        bus: &Bus,
    ) -> CommandResult {
        if let Some(status_type) = StatusType::parse(&input[1..]) {
            match status_type {
                StatusType::Address(address) => println!("0x{:X}", bus.read(address)),
                StatusType::Register(register) => {
                    println!("0x{:X}", debug_info.registers.reg(register))
                }
                StatusType::Registers => println!("{:?}", debug_info.registers),
            }
        } else {
            println!(
                "Invalid command for status (address [address] | register [register] | registers)"
            )
        }

        CommandResult::Continue
    }
}
