use super::{Command, CommandResult, Debugger};
use crate::util::parse_register;
use rustyboy_core::debugger::breakpoint::{Breakpoint, BreakpointCondition};
use rustyboy_core::debugger::commands::breakpoint::BreakpointAction;
use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::debugger::DebuggerAction;
use rustyboy_core::util::parse_hex::parse_hex;

const MATCHING_VALUES: &[&str] = &["breakpoint", "b"];

#[derive(Clone, PartialEq, Debug)]
pub enum BreakpointCommandAction {
    BreakpointAction(BreakpointAction),
    List,
}

impl BreakpointCommandAction {
    pub fn parse(values: &[&str]) -> Option<BreakpointCommandAction> {
        let action = *values.get(0)?;
        match action {
            "add" | "a" => {
                let breakpoint = parse_breakpoint(&values[1..values.len()])?;
                Some(BreakpointCommandAction::BreakpointAction(
                    BreakpointAction::Add(breakpoint),
                ))
            }
            "remove" | "r" => {
                let index: usize = values.get(1)?.parse().ok()?;
                Some(BreakpointCommandAction::BreakpointAction(
                    BreakpointAction::Remove(index),
                ))
            }
            "list" | "l" => Some(BreakpointCommandAction::List),
            _ => None,
        }
    }
}

fn parse_breakpoint(values: &[&str]) -> Option<Breakpoint> {
    let conditions = if !values.is_empty() {
        let conditions: Vec<Option<BreakpointCondition>> =
            values.iter().map(|x| parse_condition(x)).collect();

        if conditions.iter().any(|x| x.is_none()) {
            return None;
        } else {
            conditions.iter().map(|x| x.unwrap()).collect()
        }
    } else {
        vec![]
    };

    Some(Breakpoint { conditions })
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

    fn execute(&self, input: &[&str], debugger: &mut Debugger, _: &DebugInfo) -> CommandResult {
        if let Some(action) = BreakpointCommandAction::parse(&input[1..]) {
            match action {
                BreakpointCommandAction::BreakpointAction(action) => {
                    debugger.run_action(DebuggerAction::Breakpoint(action));
                }
                BreakpointCommandAction::List => println!("{}", list_breakpoints(debugger)),
            }
        } else {
            println!("Invalid input for breakpoint (add [line]| remove [line] | list)");
        }
        CommandResult::Continue
    }
}

fn list_breakpoints(debugger: &Debugger) -> String {
    if debugger.breakpoints.is_empty() {
        "No breakpoints set".to_string()
    } else {
        debugger
            .breakpoints
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (idx, breakpoint)| {
                if idx == 0 {
                    format!("{:?}", breakpoint)
                } else {
                    format!("{}, {:?}", acc, breakpoint)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::shell_debugger::commands::breakpoint::BreakpointCommandAction;
    use rustyboy_core::debugger::breakpoint::{Breakpoint, BreakpointCondition};
    use rustyboy_core::debugger::commands::breakpoint::BreakpointAction;
    use rustyboy_core::processor::registers::RegisterType;

    #[test]
    fn parses_breakpoint_correctly() {
        let input = ["b", "a", "pc=0x1e7e"];
        assert_eq!(
            BreakpointCommandAction::parse(&input[1..]).unwrap(),
            BreakpointCommandAction::BreakpointAction(BreakpointAction::Add(Breakpoint {
                conditions: vec![BreakpointCondition::RegisterEquals(RegisterType::PC, 0x1E7E)]
            })),
        );
    }

    #[test]
    fn parses_breakpoint_with_condition_correctly() {
        let input = ["b", "a", "hl=0x1e7e"];
        assert_eq!(
            BreakpointCommandAction::parse(&input[1..]).unwrap(),
            BreakpointCommandAction::BreakpointAction(BreakpointAction::Add(Breakpoint {
                //                line: 0x1E7E,
                conditions: vec![BreakpointCondition::RegisterEquals(
                    RegisterType::HL,
                    0x1E7E
                )]
            })),
        );
    }
}
