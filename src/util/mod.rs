use std::string::FromUtf8Error;

pub mod bytes_convert;
pub mod bits;

pub fn ut8_decode_trim(buffer: Vec<u8>) -> Result<String, FromUtf8Error> {
    return String::from_utf8(
        buffer.iter()
            .filter(|&&x| {
                x != 0
            })
            .map(|x| *x)
            .collect()
    );
}