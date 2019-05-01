use super::Command;
use crate::bus::Bus;
use crate::debugger::commands::CommandResult;
use crate::debugger::{DebugInfo, DebuggerState, Breakpoint};
use crate::util::parse_hex::parse_hex;

const MATCHING_VALUES: &'static [&'static str] = &["breakpoint", "b"];

pub enum BreakpointAction {
    Add(Breakpoint),
    Remove(u16),
    List,
}

impl BreakpointAction {
    pub fn parse(values: &[&str]) -> Option<BreakpointAction> {
        let action = *values.get(0)?;
        match action {
            "add" | "a" => {
                let breakpoint = Breakpoint {
                    line: parse_hex(values.get(1)?)?,
                    condition: None
                };
                Some(BreakpointAction::Add(breakpoint))
            }
            "remove" | "r" => {
                let line: u16 = parse_hex(values.get(1)?)?;
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
                BreakpointAction::Add(breakpoint) => debugger.breakpoints.push(breakpoint),
                BreakpointAction::Remove(line) => {
                    debugger.breakpoints = debugger.breakpoints.iter().filter(|b| b.line != line).cloned().collect();
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
    if debugger.breakpoints.is_empty() {
        "No breakpoints set".to_string()
    } else {
        debugger
            .breakpoints
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (idx, breakpoint)| {
                if idx == 0 {
                    format!("0x{:X}", breakpoint.line)
                } else {
                    format!("{}, 0x{:X}", acc, breakpoint.line)
                }
            })
    }
}
