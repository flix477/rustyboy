pub fn parse_hex(value: &str) -> Option<u16> {
    let lowercased = value.to_lowercase();
    let lol = value.trim_start_matches("0x");
    u16::from_str_radix(value, 16).ok()
}