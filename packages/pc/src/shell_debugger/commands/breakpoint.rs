use super::{Command, CommandResult, DebuggerState};
use crate::shell_debugger::breakpoint::{Breakpoint, BreakpointCondition};
use crate::util::parse_register;
use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::util::parse_hex::parse_hex;

const MATCHING_VALUES: &[&str] = &["breakpoint", "b"];

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
                let breakpoint = parse_breakpoint(&values[1..values.len()])?;
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

fn parse_breakpoint(values: &[&str]) -> Option<Breakpoint> {
    let line = parse_hex(values.get(0)?)?;
    let conditions = if *values.get(1)? == "if" {
        if values.len() < 3 {
            return None;
        }

        let rest = (&values[2..values.len()]).join(" ").to_lowercase();

        let conditions: Vec<Option<BreakpointCondition>> =
            rest.split(" and ").map(|x| parse_condition(x)).collect();

        if conditions.iter().any(|x| x.is_none()) {
            return None;
        } else {
            Some(conditions.iter().map(|x| x.unwrap()).collect())
        }
    } else {
        None
    };

    Some(Breakpoint { line, conditions })
}

fn parse_condition(value: &str) -> Option<BreakpointCondition> {
    let parts: Vec<&str> = value.split('=').collect();
    let object = parts.get(0)?;
    let value = parse_hex(parts.get(1)?)?;

    if let Some(register) = parse_register(object) {
        Some(BreakpointCondition::RegisterEquals(register, value))
    } else {
        None
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
        _: &DebugInfo<'_>,
        _: &dyn Bus,
    ) -> CommandResult {
        if let Some(action) = BreakpointAction::parse(&input[1..]) {
            match action {
                BreakpointAction::Add(breakpoint) => debugger.breakpoints.push(breakpoint),
                BreakpointAction::Remove(line) => {
                    debugger.breakpoints = debugger
                        .breakpoints
                        .iter()
                        .filter(|b| b.line != line)
                        .cloned()
                        .collect();
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
