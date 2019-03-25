use crate::processor::registers::RegisterType;

pub enum StatusType {
    Address(u16),
    Register(RegisterType),
    Registers,
}

impl StatusType {
    pub fn parse(values: Vec<&str>) -> Option<StatusType> {
        let status = *values.get(0)?;
        match status {
            "register" | "r" => {
                let register = values.get(1)?.to_lowercase().as_str();
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

pub enum Command {
    Continue,
    Help,
    Status(StatusType),
    StepInto,
    StepOver,
    Quit
}

impl Command {
    pub fn parse(value: &str) -> Option<Command> {
        let separated = value.split(' ');

        match separated[0] {
            "continue" | "c" => Some(Command::Continue),
            "help" | "h" | "?" => Some(Command::Help),
            "status" | "s" => {
                let status = StatusType::parse(separated[1..])?;
                Some(Command::Status(status))
            }
            "step_into" | "si" => Some(Command::StepInto),
            "step_over" | "so" => Some(Command::StepOver),
            "quit" | "q" => Some(Command::Quit),
            _ => None
        }
    }
}