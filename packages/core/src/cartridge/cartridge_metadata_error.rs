use thiserror;
use std::string::FromUtf8Error;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum CartridgeMetadataError {
    #[error("Invalid ROM size index")]
    InvalidROMSizeIndex,
    #[error("Invalid RAM size index")]
    InvalidRAMSizeIndex,
    #[error("Invalid destination code")]
    InvalidDestinationCode,
    #[error("Invalid cartridge type value")]
    InvalidCartridgeTypeValue,
    #[error("utf8 error")]
    Utf8Error,
    #[error("io error")]
    IOError,
}

impl From<FromUtf8Error> for CartridgeMetadataError {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8Error
    }
}

impl From<io::Error> for CartridgeMetadataError {
    fn from(value: io::Error) -> Self {
        Self::IOError
    }
}
