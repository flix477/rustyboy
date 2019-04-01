use std::io::{self, Read};
use crate::bus::Bus;
use crate::processor::Processor;
use std::collections::HashSet;
use crate::debugger::commands::{Command, CommandResult};

pub mod commands;

pub struct Debugger<'a> {
    pub state: DebuggerState,
    pub commands: Vec<Command<'a>>
}

pub struct DebuggerState {
    pub breakpoints: HashSet<u16>
}

impl<'a> Debugger<'a> {
    pub fn new() -> Debugger<'a> {
        Debugger {
            state: DebuggerState { breakpoints: HashSet::new() },
            commands: vec![
                commands::breakpoint::create_command(),
                commands::status::create_command()
            ]
        }
    }

    pub fn run(&mut self, cpu: &Processor, bus: &Bus) {
        loop {
            if let Some(result) = self.parse(cpu, bus) {
                match result {
                    CommandResult::Continue => { break; },
                    CommandResult::Quit => { std::process::exit(0) },
                    _ => {}
                }
            } else {
                println!("Invalid command");
            }
        }
    }

    fn parse(&mut self, cpu: &Processor, bus: &Bus) -> Option<CommandResult> {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input);
        let separated: Vec<&str> = input.split(' ').collect();
        let command = matching_command(&self.commands, separated[0])?;
        Some((command.callback)(&separated, &mut self.state, cpu, bus))
    }

}

fn matching_command(commands: &Vec<Command>, value: &str) -> Option<&Command> {
    commands.iter().find(|cmd| cmd.matching_values.contains(&value))
}
