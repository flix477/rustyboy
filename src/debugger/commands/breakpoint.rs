use super::Command;
use crate::bus::Bus;
use crate::debugger::commands::{CommandResult, Debugger};
use crate::debugger::{DebugInfo, DebuggerState};
use crate::processor::registers::Registers;
use crate::processor::Processor;

const MATCHING_VALUES: &'static [&'static str] = &["breakpoint", "b"];

pub enum BreakpointAction {
    Add(u16),
    Remove(u16),
    List,
}

impl BreakpointAction {
    pub fn parse(values: &[&str]) -> Option<BreakpointAction> {
        let action = *values.get(0)?;
        match action {
            "add" | "a" => {
                let line: u16 = values.get(1)?.parse().ok()?;
                Some(BreakpointAction::Add(line))
            }
            "remove" | "r" => {
                let line: u16 = values.get(1)?.parse().ok()?;
                Some(BreakpointAction::Remove(line))
            }
            "list" | "l" => Some(BreakpointAction::List),
            _ => None,
        }
    }
}

pub struct BreakpointCommand {}

impl BreakpointCommand {
    pub fn create_command() -> Box<dyn Command> {
        Box::new(BreakpointCommand {})
    }
}

impl Command for BreakpointCommand {
    fn matching_value(&self) -> &[&str] {
        MATCHING_VALUES
    }

    fn execute(
        &self,
        input: &[&str],
        debugger: &mut DebuggerState,
        _: &DebugInfo,
        _: &Bus,
    ) -> CommandResult {
        if let Some(action) = BreakpointAction::parse(&input[1..]) {
            match action {
                BreakpointAction::Add(line) => {
                    debugger.breakpoints.insert(line);
                }
                BreakpointAction::Remove(line) => {
                    debugger.breakpoints.remove(&line);
                }
                BreakpointAction::List => println!("{}", list_breakpoints(debugger)),
            }
        } else {
            println!("Invalid input for breakpoint (add [line]| remove [line] | list)");
        }
        CommandResult::Continue
    }
}

fn list_breakpoints(debugger: &DebuggerState) -> String {
    if debugger.breakpoints.len() == 0 {
        "No breakpoints set".to_string()
    } else {
        debugger
            .breakpoints
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (idx, value)| {
                if idx == 0 {
                    value.to_string()
                } else {
                    format!("{}, {}", acc, value)
                }
            })
    }
}
