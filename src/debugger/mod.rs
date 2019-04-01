use self::commands::breakpoint::BreakpointCommand;
use self::commands::status::StatusCommand;
use self::commands::step_into::StepIntoCommand;
use self::commands::step_over::StepOverCommand;
use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult};
use crate::processor::instruction::InstructionInfo;
use crate::processor::registers::Registers;
use std::collections::HashSet;
use std::fmt::{Debug, Error, Formatter};
use std::io::{self, Read};

pub mod commands;

pub struct Debugger {
    pub state: DebuggerState,
    pub commands: Vec<Box<dyn Command>>,
}

#[derive(Clone)]
pub struct DebuggerState {
    pub breakpoints: HashSet<u16>,
    pub forced_break: bool,
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self {
            breakpoints: HashSet::new(),
            forced_break: false,
        }
    }
}

pub struct DebugInfo<'a> {
    pub registers: &'a Registers,
    pub line: u16,
    pub instruction: &'a InstructionInfo,
}

impl<'a> Debug for DebugInfo<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "0x{:X}: {:?}", self.line, self.instruction.mnemonic())
    }
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            state: DebuggerState::default(),
            commands: vec![
                BreakpointCommand::create_command(),
                StatusCommand::create_command(),
                StepIntoCommand::create_command(),
                StepOverCommand::create_command(),
            ],
        }
    }

    pub fn from_state(state: DebuggerState) -> Debugger {
        Debugger {
            state,
            commands: vec![
                BreakpointCommand::create_command(),
                StatusCommand::create_command(),
                StepIntoCommand::create_command(),
                StepOverCommand::create_command(),
            ],
        }
    }

    pub fn run(&mut self, debug_info: DebugInfo, bus: &Bus) {
        if self.state.forced_break {
            self.state.forced_break = false;
        }
        println!("{:?}", debug_info);
        loop {
            if let Some(result) = self.parse(&debug_info, bus) {
                match result {
                    CommandResult::Continue => {
                        break;
                    }
                    CommandResult::Quit => std::process::exit(0),
                    _ => {}
                }
            } else {
                println!("Invalid command");
            }
        }
    }

    fn help(&self) {
        println!(
            "Available commands:{}",
            self.commands
                .iter()
                .fold(String::new(), |acc, command| format!(
                    "{}\n{}",
                    acc,
                    command.help()
                ))
        )
    }

    pub fn should_run(&self, line: u16) -> bool {
        self.state.breakpoints.contains(&line) || self.state.forced_break
    }

    fn parse(&mut self, debug_info: &DebugInfo, bus: &Bus) -> Option<CommandResult> {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input);
        let separated: Vec<&str> = input.split(' ').collect();
        let command = matching_command(&self.commands, separated[0].to_string())?;
        Some(command.execute(&separated, &mut self.state, debug_info, bus))
    }
}

fn matching_command(commands: &Vec<Box<dyn Command>>, value: String) -> Option<&Box<dyn Command>> {
    commands
        .iter()
        .find(|cmd| cmd.matching_value().contains(&value.as_str()))
}
