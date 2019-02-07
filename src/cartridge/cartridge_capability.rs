#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CartridgeCapability {
    ROM,
    MBC1,
    MBC2,
    MBC3,
    MBC4,
    MBC5,
    RAM,
    Battery,
    Timer,
    MMM01,
    Rumble,
    PocketCamera,
    BandaiTama5,
    HuC1,
    HuC3,
}

impl CartridgeCapability {
    pub fn from_byte(value: u8) -> Result<Vec<CartridgeCapability>, String> {
        return match value {
            0x00 => Ok(vec![CartridgeCapability::ROM]),
            0x01 => Ok(vec![CartridgeCapability::MBC1]),
            0x02 => Ok(vec![CartridgeCapability::MBC1, CartridgeCapability::RAM]),
            0x03 => Ok(vec![
                CartridgeCapability::MBC1,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x05 => Ok(vec![CartridgeCapability::MBC2]),
            0x06 => Ok(vec![
                CartridgeCapability::MBC2,
                CartridgeCapability::Battery,
            ]),
            0x08 => Ok(vec![CartridgeCapability::ROM, CartridgeCapability::RAM]),
            0x09 => Ok(vec![
                CartridgeCapability::ROM,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x0B => Ok(vec![CartridgeCapability::MMM01]),
            0x0C => Ok(vec![CartridgeCapability::MMM01, CartridgeCapability::RAM]),
            0x0D => Ok(vec![
                CartridgeCapability::MMM01,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x0F => Ok(vec![
                CartridgeCapability::MBC3,
                CartridgeCapability::Timer,
                CartridgeCapability::Battery,
            ]),
            0x10 => Ok(vec![
                CartridgeCapability::MBC3,
                CartridgeCapability::Timer,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x11 => Ok(vec![CartridgeCapability::MBC3]),
            0x12 => Ok(vec![CartridgeCapability::MBC3, CartridgeCapability::RAM]),
            0x13 => Ok(vec![
                CartridgeCapability::MBC3,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x15 => Ok(vec![CartridgeCapability::MBC4]),
            0x16 => Ok(vec![CartridgeCapability::MBC4, CartridgeCapability::RAM]),
            0x17 => Ok(vec![
                CartridgeCapability::MBC4,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x19 => Ok(vec![CartridgeCapability::MBC5]),
            0x1A => Ok(vec![CartridgeCapability::MBC5, CartridgeCapability::RAM]),
            0x1B => Ok(vec![
                CartridgeCapability::MBC5,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0x1C => Ok(vec![CartridgeCapability::MBC5, CartridgeCapability::Rumble]),
            0x1D => Ok(vec![
                CartridgeCapability::MBC5,
                CartridgeCapability::Rumble,
                CartridgeCapability::RAM,
            ]),
            0x1E => Ok(vec![
                CartridgeCapability::MBC5,
                CartridgeCapability::Rumble,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            0xFC => Ok(vec![CartridgeCapability::PocketCamera]),
            0xFD => Ok(vec![CartridgeCapability::BandaiTama5]),
            0xFE => Ok(vec![CartridgeCapability::HuC3]),
            0xFF => Ok(vec![
                CartridgeCapability::HuC1,
                CartridgeCapability::RAM,
                CartridgeCapability::Battery,
            ]),
            _ => Err(String::from("invalid cartridge type value")),
        };
    }
}
