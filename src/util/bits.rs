pub fn get_bit(value: u8, idx: u8) -> bool {
    value >> (7 - idx) == 1
}

pub fn set_bit(value: u8, idx: u8, new_value: bool) -> u8 {
    let padded_value = (value as u8) << idx;
    if new_value {
        value | padded_value
    } else {
        value & !2u8.pow(idx as u32)
    }
}