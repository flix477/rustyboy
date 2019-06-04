pub fn get_bit(value: u8, idx: u8) -> bool {
    (value >> idx) & 1 != 0
}

pub fn set_bit(value: u8, idx: u8, new_value: bool) -> u8 {
    let padded_value = 1u8 << idx;
    if new_value {
        value | padded_value
    } else {
        value & !2u8.pow(idx as u32)
    }
}
