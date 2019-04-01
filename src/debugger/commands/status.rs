use std::collections::HashSet;
use std::io::{self, Read};

use crate::debugger::commands::{Command, CommandResult, Debugger};
use crate::processor::lr35902::LR35902;
use crate::processor::registers::RegisterType;
use crate::processor::Processor;
use crate::bus::Bus;

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
            },
            "address" | "a" => {
                None
            }
            _ => None
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
            _ => None
        }
    }
}

pub fn create_command<'a>() -> Command<'a> {
    Command {
        matching_values: vec!["status", "s"],
        callback: Box::new(|values, _, cpu, bus| {
            execute_command(values, cpu, bus);
            CommandResult::None
        })
    }
}

fn execute_command(values: &Vec<&str>, cpu: &Processor, bus: &Bus) {
    if let Some(status_type) = StatusType::parse(values) {
        match status_type {
            StatusType::Address(address) => println!("0x{:X}", bus.read(address)),
            StatusType::Register(register) => println!("0x{:X}", cpu.reg(register)),
            StatusType::Registers => {
                println!(
                    "AF: 0x{:X}\nBC: 0x{:X}\nDE: 0x{:X}\nHL: 0x{:X}\nSP: 0x{:X}\nPC: 0x{:X}",
                    cpu.reg(RegisterType::AF),
                    cpu.reg(RegisterType::BC),
                    cpu.reg(RegisterType::DE),
                    cpu.reg(RegisterType::HL),
                    cpu.reg(RegisterType::SP),
                    cpu.reg(RegisterType::PC)
                );
            }
        }
    } else {
        println!("Invalid command for status (address [address] | register [register] | registers)")
    }
}