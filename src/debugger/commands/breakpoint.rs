use super::Command;
use crate::debugger::commands::{CommandResult, Debugger};
use crate::debugger::DebuggerState;

pub enum BreakpointAction {
    Add(u16),
    Remove(u16),
    List
}

impl BreakpointAction {
    pub fn parse(values: &[&str]) -> Option<BreakpointAction> {
        let action = *values.get(0)?;
        match action {
            "add" | "a" => {
                None
            },
            "remove" | "r" => {
                None
            },
            "list" | "l" => Some(BreakpointAction::List),
            _ => None
        }
    }
}

fn list_breakpoints(debugger: &DebuggerState) -> String {
    debugger.breakpoints.iter()
        .fold(String::new(), |acc, value| format!("{}, {}", acc, value))
}

fn execute_command(values: &Vec<&str>, debugger: &mut DebuggerState) {
    if let Some(action) = BreakpointAction::parse(&values[1..]) {
        match action {
            BreakpointAction::Add(address) => { debugger.breakpoints.insert(address); },
            BreakpointAction::Remove(address) => { debugger.breakpoints.remove(&address); },
            BreakpointAction::List => println!("{}", list_breakpoints(debugger))
        }
    } else {
        println!("Invalid input for breakpoint (add [address]| remove [address] | list)");
    }
}

pub fn create_command<'a>() -> Command<'a> {
    Command {
        matching_values: vec!["breakpoint", "b"],
        callback: Box::new(|values, debugger, _, _| {
            execute_command(values, debugger);
            CommandResult::None
        })
    }
}
