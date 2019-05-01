use crate::bus::Bus;
use crate::debugger::commands::{Command, CommandResult};

use self::commands::breakpoint::BreakpointCommand;
use self::commands::continue_cmd::ContinueCommand;
use self::commands::quit::QuitCommand;
use self::commands::status::StatusCommand;
use self::commands::step_into::StepIntoCommand;
use self::commands::step_over::StepOverCommand;
use self::debug_info::DebugInfo;
use self::shell::Shell;
use crate::processor::registers::RegisterType;

pub mod commands;
pub mod debug_info;
pub mod shell;

const HEADER: &'static str = "-- Rustyboy Debugger --";

pub struct Debugger {
    pub state: DebuggerState,
    pub commands: Vec<Box<dyn Command>>,
    shell: Shell
}

#[derive(Clone)]
pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
    pub forced_break: bool,
}

#[derive(Copy, Clone)]
pub struct Breakpoint {
    pub line: u16,
    pub condition: Option<BreakpointCondition>
}

#[derive(Copy, Clone)]
pub enum BreakpointCondition {
    RegisterEquals(RegisterType, u16)
}

impl BreakpointCondition {
    pub fn satisfied(&self, debug_info: &DebugInfo) -> bool {
        match self {
            BreakpointCondition::RegisterEquals(register, value) => {
                debug_info.registers.reg(*register) == *value
            }
        }
    }
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self {
            breakpoints: Vec::new(),
            forced_break: false,
        }
    }
}

impl Debugger {
    pub fn from_state(state: DebuggerState) -> Debugger {
        Debugger {
            state,
            commands: vec![
                BreakpointCommand::create_command(),
                ContinueCommand::create_command(),
                StatusCommand::create_command(),
                StepIntoCommand::create_command(),
                StepOverCommand::create_command(),
                QuitCommand::create_command(),
            ],
            shell: Shell::new()
        }
    }

    pub fn run(&mut self, debug_info: DebugInfo, bus: &Bus) {
        if self.state.forced_break {
            self.state.forced_break = false;
        } else {
            println!("{}", HEADER);
        }

        println!("{:?}", debug_info);

        loop {
            let input = self.shell.read_input();
            if let Some(result) = self.parse(&input, &debug_info, bus) {
                match result {
                    CommandResult::Continue => {}
                    CommandResult::Quit => {
                        break;
                    }
                }
            } else {
                println!("Invalid command");
                self.help();
            }
        }
    }

    fn help(&self) {
        println!(
            "Available commands:{}",
            self.commands
                .iter()
                .fold(String::new(), |acc, command| format!(
                    "{}\n\t{}",
                    acc,
                    command.help()
                ))
        )
    }

    pub fn should_run(&self, debug_info: &DebugInfo) -> bool {
        self.state.forced_break
            || self.state.breakpoints.iter()
                .any(|b| b.line == debug_info.line && b.condition.map_or(true, |condition| condition.satisfied(debug_info)))
    }

    fn parse(&mut self, input: &str, debug_info: &DebugInfo, bus: &Bus) -> Option<CommandResult> {
        let separated: Vec<&str> = input.split(' ').map(|x| x.trim()).collect();
        let command = matching_command(&self.commands, separated[0].to_string())?;
        Some(command.execute(&separated, &mut self.state, debug_info, bus))
    }
}

fn matching_command(commands: &[Box<dyn Command>], value: String) -> Option<&Box<dyn Command>> {
    commands
        .iter()
        .find(|cmd| cmd.matching_value().contains(&value.as_str()))
}

