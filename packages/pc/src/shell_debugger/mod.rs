use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::{debug_info::DebugInfo, Debugger};

use self::breakpoint::Breakpoint;
use self::commands::{
    breakpoint::BreakpointCommand, continue_cmd::ContinueCommand, quit::QuitCommand,
    status::StatusCommand, step_into::StepIntoCommand, step_over::StepOverCommand, Command,
    CommandResult,
};
use self::pretty_print::format_debug_info;
use self::shell::Shell;

mod breakpoint;
mod commands;
mod pretty_print;
mod shell;

const HEADER: &str = "-- Rustyboy Debugger --";

#[derive(Clone)]
pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
    pub forced_break: bool,
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self {
            breakpoints: Vec::new(),
            forced_break: false,
        }
    }
}

pub struct ShellDebugger {
    pub state: DebuggerState,
    pub commands: Vec<Box<dyn Command>>,
    shell: Shell,
}

impl ShellDebugger {
    pub fn from_state(state: DebuggerState) -> ShellDebugger {
        ShellDebugger {
            state,
            commands: vec![
                BreakpointCommand::create_command(),
                ContinueCommand::create_command(),
                StatusCommand::create_command(),
                StepIntoCommand::create_command(),
                StepOverCommand::create_command(),
                QuitCommand::create_command(),
            ],
            shell: Shell::new(),
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

    fn parse(
        &mut self,
        input: &str,
        debug_info: &DebugInfo<'_>,
        bus: &dyn Bus,
    ) -> Option<CommandResult> {
        let separated: Vec<&str> = input.split(' ').map(|x| x.trim()).collect();
        let command = matching_command(&self.commands, separated[0].to_string())?;
        Some(command.execute(&separated, &mut self.state, debug_info, bus))
    }
}

impl Debugger for ShellDebugger {
    fn should_run(&self, debug_info: &DebugInfo<'_>) -> bool {
        self.state.forced_break
            || self.state.breakpoints.iter().any(|b| {
                b.line == debug_info.line
                    && b.conditions.clone().map_or(true, |conditions| {
                        conditions
                            .iter()
                            .all(|condition| condition.satisfied(debug_info))
                    })
            })
    }

    fn run(&mut self, debug_info: DebugInfo<'_>, bus: &dyn Bus) {
        if self.state.forced_break {
            self.state.forced_break = false;
        } else {
            println!("{}", HEADER);
        }

        println!("{}", format_debug_info(&debug_info));

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
}

fn matching_command(commands: &[Box<dyn Command>], value: String) -> Option<&dyn Command> {
    commands
        .iter()
        .find(|cmd| cmd.matching_value().contains(&value.as_str()))
        .map(|cmd| cmd.as_ref())
}
