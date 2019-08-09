use rustyboy_core::debugger::{debug_info::DebugInfo, Debugger};

use self::pretty_print::format_debug_info;
use self::shell::Shell;
use crate::shell_debugger::commands::breakpoint::BreakpointCommand;
use crate::shell_debugger::commands::continue_cmd::ContinueCommand;
use crate::shell_debugger::commands::quit::QuitCommand;
use crate::shell_debugger::commands::status::StatusCommand;
use crate::shell_debugger::commands::step_into::StepIntoCommand;
use crate::shell_debugger::commands::{Command, CommandResult};

mod commands;
mod pretty_print;
mod shell;

const HEADER: &str = "-- Rustyboy Debugger --";

pub struct ShellDebugger {
    pub commands: Vec<Box<dyn Command>>,
    shell: Shell,
}

impl Default for ShellDebugger {
    fn default() -> Self {
        Self {
            commands: vec![
                BreakpointCommand::create_command(),
                ContinueCommand::create_command(),
                StatusCommand::create_command(),
                StepIntoCommand::create_command(),
                QuitCommand::create_command(),
            ],
            shell: Shell::new(),
        }
    }
}

impl ShellDebugger {
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

    fn parse(
        &self,
        input: &str,
        debugger: &mut Debugger,
        debug_info: &DebugInfo,
    ) -> Option<CommandResult> {
        let separated: Vec<&str> = input.split(' ').map(|x| x.trim()).collect();
        let command = matching_command(&self.commands, separated[0].to_string())?;
        Some(command.execute(&separated, debugger, debug_info))
    }

    pub fn run(&mut self, debugger: &mut Debugger, debug_info: &DebugInfo) {
        if debugger.forced_break {
            debugger.forced_break = false;
        } else {
            println!("{}", HEADER);
        }

        println!("{}", format_debug_info(&debug_info));

        loop {
            let input = self.shell.read_input();
            if let Some(result) = self.parse(&input, debugger, &debug_info) {
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
}

fn matching_command(commands: &[Box<dyn Command>], value: String) -> Option<&dyn Command> {
    commands
        .iter()
        .find(|cmd| cmd.matching_value().contains(&value.as_str()))
        .map(|cmd| cmd.as_ref())
}
