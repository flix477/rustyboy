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
            "add" | "a" => None,
            "remove" | "r" => None,
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
                BreakpointAction::Add(address) => {
                    debugger.breakpoints.insert(address);
                }
                BreakpointAction::Remove(address) => {
                    debugger.breakpoints.remove(&address);
                }
                BreakpointAction::List => println!("{}", list_breakpoints(debugger)),
            }
        } else {
            println!("Invalid input for breakpoint (add [address]| remove [address] | list)");
        }
        CommandResult::None
    }
}

fn list_breakpoints(debugger: &DebuggerState) -> String {
    debugger
        .breakpoints
        .iter()
        .fold(String::new(), |acc, value| format!("{}, {}", acc, value))
}
