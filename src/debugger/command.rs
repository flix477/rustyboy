use crate::processor::registers::RegisterType;
use crate::debugger::command::Command::Breakpoint;
use crate::processor::Processor;
use crate::bus::Bus;

pub enum StatusType {
    Address(u16),
    Register(RegisterType),
    Registers,
}

impl StatusType {
    pub fn parse(values: &[&str]) -> Option<StatusType> {
        let status = *values.get(0)?;
        match status {
            "register" | "r" => {
                let register = values.get(1)?;
                let register = StatusType::parse_register(register)?;
                Some(StatusType::Register(register))
            },
            "address" | "a" => {
                None
            }
            _ => None
        }
    }

    fn parse_register(value: &str) -> Option<RegisterType> {
        match value {
            "a" => Some(RegisterType::A),
            "f" => Some(RegisterType::F),
            "af" => Some(RegisterType::AF),
            "b" => Some(RegisterType::B),
            "c" => Some(RegisterType::C),
            "bc" => Some(RegisterType::BC),
            "d" => Some(RegisterType::D),
            "e" => Some(RegisterType::E),
            "de" => Some(RegisterType::DE),
            "sp" => Some(RegisterType::SP),
            "pc" => Some(RegisterType::PC),
            _ => None
        }
    }
}

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

pub enum Command {
    Continue,
    Help,
    Breakpoint(BreakpointAction),
    Status(StatusType),
    StepInto,
    StepOver,
    Quit
}

impl Command {
    pub fn parse(value: &str) -> Option<Command> {
        let separated: Vec<&str> = value.split(' ').collect();

        match separated[0] {
            "continue" | "c" => Some(Command::Continue),
            "help" | "h" | "?" => Some(Command::Help),
            "breakpoint" | "b" => {
                let action = BreakpointAction::parse(&separated[1..])?;
                Some(Command::Breakpoint(action))
            }
            "status" | "s" => {
                let status = StatusType::parse(&separated[1..])?;
                Some(Command::Status(status))
            }
            "step_into" | "si" => Some(Command::StepInto),
            "step_over" | "so" => Some(Command::StepOver),
            "quit" | "q" => Some(Command::Quit),
            _ => None
        }
    }
}

struct Debugger {
    pub breakpoints: Vec<u16>
}

struct Command2 {
    pub matching_values: Vec<&'static str>,
    pub callback: Box<FnMut(&mut Debugger, &Processor, &Bus)>
}

const LOL: Command2 = Command2 {
    matching_values: vec!["lol"],
    callback: Box::new(|debugger, cpu, bus| {})
};