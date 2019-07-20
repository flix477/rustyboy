use std::string::FromUtf8Error;

pub mod bitflags;
pub mod bits;
pub mod bytes_convert;
pub mod drawer;
pub mod parse_hex;
#[cfg(test)]
pub mod tests;

pub fn ut8_decode_trim(buffer: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(buffer.iter().filter(|&&x| x != 0).cloned().collect())
}

pub fn wrap_value(value: usize, max: usize) -> usize {
    if value >= max {
        value - max
    } else {
        value
    }
}
