use std::fs;
use std::error::Error;
use cartridge::cartridge_metadata::CartridgeMetadata;

pub struct Cartridge {
    buffer: Vec<u8>,
    metadata: CartridgeMetadata
}

impl Cartridge {
    pub fn from_file(filename: &str) -> Result<Cartridge, Box<dyn Error>> {
        return Cartridge::from_buffer(fs::read(filename)?);
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<Cartridge, Box<dyn Error>> {
        return Ok(Cartridge {
            metadata: CartridgeMetadata::from_buffer(&buffer)?,
            buffer
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_pokemon_blue() {
        assert!(Cartridge::from_file("pokemonb.gb").is_ok());
    }
}