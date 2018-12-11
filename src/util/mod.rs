use std::string::FromUtf8Error;
use std::time::Duration;

pub mod bytes_convert;
pub mod bits;
pub mod bitflags;

pub fn ut8_decode_trim(buffer: Vec<u8>) -> Result<String, FromUtf8Error> {
    return String::from_utf8(
        buffer.iter()
            .filter(|&&x| {
                x != 0
            })
            .cloned()
            .collect()
    );
}

pub fn as_millis(duration: Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0
}