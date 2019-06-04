pub fn parse_hex(value: &str) -> Option<u16> {
    let lowercased = value.to_lowercase();
    let trimmed = lowercased.trim_start_matches("0x");

    u16::from_str_radix(trimmed, 16).ok()
}
