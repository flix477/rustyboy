use rustyboy_core::processor::registers::RegisterType;

pub fn parse_register(value: &str) -> Option<RegisterType> {
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
        "h" => Some(RegisterType::H),
        "l" => Some(RegisterType::L),
        "hl" => Some(RegisterType::HL),
        "sp" => Some(RegisterType::SP),
        "pc" => Some(RegisterType::PC),
        _ => None,
    }
}
