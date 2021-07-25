use crate::cartridge::cartridge_capability::CartridgeCapability;
use crate::util::bytes_convert;
use crate::util::ut8_decode_trim;
use std::error::Error;
use std::ops::RangeInclusive;

// The range where the game's title resides in ASCII uppercase characters
const GAME_TITLE_OFFSET: usize = 0x0134;

// The range where the manufacturer code resides
// for newer cartridges, 4 characters uppercase
const MANUFACTURER_CODE_RANGE: RangeInclusive<usize> = 0x013F..=0x0142;

// The offset on CGB cartridges to determine which CGB functions they support
const CGB_FLAG_OFFSET: usize = 0x0143;

// The range where the company/publisher code of
// the game is in post-SGB cartridges
const NEW_LICENSEE_CODE_RANGE: RangeInclusive<usize> = 0x144..=0x145;

// The offset that determines whether the game has SGB functions
const SGB_FLAG_OFFSET: usize = 0x0146;
const SBG_FLAG_ENABLED: u8 = 0x03;

// The offset that determines the cartridge type
const CARTRIDGE_TYPE_OFFSET: usize = 0x0147;

// The offset that determines the ROM size
const ROM_SIZE_OFFSET: usize = 0x0148;

// The offset that determines the external RAM size (if any)
const RAM_SIZE_OFFSET: usize = 0x0149;

// The offset that determines the game's destination market
const DESTINATION_OFFSET: usize = 0x014A;

// The range where the company/publisher code of
// the game is in pre-SGB cartridges
const OLD_LICENSEE_CODE_OFFSET: usize = 0x014B;

// The offset that determines the game's version number
const VERSION_OFFSET: usize = 0x014C;

// Offset that determines the checksum of the cartridge header (0x0134..=014C),
// obtained with "x=0:FOR i=0134h TO 014Ch:x=x-MEM[i]-1:NEXT"
const HEADER_CHECKSUM_OFFSET: usize = 0x014D;

// Offset that determines the checksum of the whole cartridge ROM,
// obtained by summing the bytes of the cartridge (without checksum)
// const GLOBAL_CHECKSUM_RANGE: RangeInclusive<usize> = (0x014E..=0x014F);

type TitleSection = (String, Option<String>, Option<CGBFlag>);

#[derive(Debug)]
pub struct CartridgeMetadata {
    pub title: String,
    pub manufacturer_code: Option<String>,
    pub cgb_flag: Option<CGBFlag>,
    pub new_licensee_code: Option<String>,
    pub sgb_enhanced: bool,
    pub capabilities: Vec<CartridgeCapability>,
    pub rom_size: usize,
    pub ram_size: usize,
    pub destination: Destination,
    pub old_licensee_code: Option<u8>,
    pub version: u8,
    pub header_checksum: u8,
    pub global_checksum: u8,
}

impl CartridgeMetadata {
    pub fn from_buffer(buffer: &[u8]) -> Result<CartridgeMetadata, Box<dyn Error>> {
        let (title, manufacturer_code, cgb_flag) = Self::parse_title_section(buffer)?;

        Ok(CartridgeMetadata {
            title,
            manufacturer_code,
            cgb_flag,
            new_licensee_code: Self::parse_new_licensee_code(buffer)?,
            sgb_enhanced: buffer[SGB_FLAG_OFFSET] == SBG_FLAG_ENABLED,
            capabilities: CartridgeCapability::from_byte(buffer[CARTRIDGE_TYPE_OFFSET])?,
            rom_size: Self::parse_rom_size(buffer)? as usize,
            ram_size: Self::parse_ram_size(buffer)? as usize,
            destination: Destination::from(buffer[DESTINATION_OFFSET])?,
            old_licensee_code: Self::parse_old_licensee_code(buffer),
            version: buffer[VERSION_OFFSET],
            header_checksum: buffer[HEADER_CHECKSUM_OFFSET],
            global_checksum: 0, // TODO
        })
    }

    fn parse_title_section(buffer: &[u8]) -> Result<TitleSection, Box<dyn Error>> {
        let mut title_end_offset = CGB_FLAG_OFFSET;
        let cgb_flag = CGBFlag::from(buffer[CGB_FLAG_OFFSET]);
        let manufacturer_code = Self::parse_manufacturer_code(buffer)?;
        // I might be wrong about this, but I'm currently assuming that
        // only games that have the CGB flag have a manufacturer code
        if cgb_flag.is_some() {
            title_end_offset -= 1;
            if manufacturer_code.is_some() {
                title_end_offset -= 4;
            }
        }
        Ok((
            ut8_decode_trim(buffer[(GAME_TITLE_OFFSET..title_end_offset)].to_vec())?,
            manufacturer_code,
            cgb_flag,
        ))
    }

    fn parse_manufacturer_code(buffer: &[u8]) -> Result<Option<String>, Box<dyn Error>> {
        let code = ut8_decode_trim(buffer[MANUFACTURER_CODE_RANGE].to_vec())?;
        if code.len() == 4 {
            return Ok(Some(code));
        }
        Ok(None)
    }

    fn parse_new_licensee_code(buffer: &[u8]) -> Result<Option<String>, Box<dyn Error>> {
        let code = ut8_decode_trim(buffer[NEW_LICENSEE_CODE_RANGE].to_vec())?;
        if code.len() == 2 {
            return Ok(Some(code));
        }
        Ok(None)
    }

    fn parse_rom_size(buffer: &[u8]) -> Result<f64, String> {
        match buffer[ROM_SIZE_OFFSET] {
            0x00 => Ok(bytes_convert::from_kb(32.0)),
            0x01 => Ok(bytes_convert::from_kb(64.0)),
            0x02 => Ok(bytes_convert::from_kb(128.0)),
            0x03 => Ok(bytes_convert::from_kb(256.0)),
            0x04 => Ok(bytes_convert::from_kb(512.0)),
            0x05 => Ok(bytes_convert::from_mb(1.0)),
            0x06 => Ok(bytes_convert::from_mb(2.0)),
            0x07 => Ok(bytes_convert::from_mb(4.0)),
            0x52 => Ok(bytes_convert::from_mb(1.1)),
            0x53 => Ok(bytes_convert::from_mb(1.2)),
            0x54 => Ok(bytes_convert::from_mb(1.5)),
            _ => Err(String::from("invalid ROM size value")),
        }
    }

    fn parse_ram_size(buffer: &[u8]) -> Result<f64, String> {
        match buffer[RAM_SIZE_OFFSET] {
            0x00 => Ok(0.0),
            0x01 => Ok(bytes_convert::from_kb(2.0)),
            0x02 => Ok(bytes_convert::from_kb(8.0)),
            0x03 => Ok(bytes_convert::from_kb(32.0)),
            _ => Err(String::from("invalid RAM size value")),
        }
    }

    fn parse_old_licensee_code(buffer: &[u8]) -> Option<u8> {
        let value = buffer[OLD_LICENSEE_CODE_OFFSET];
        match value {
            0x33 => None,
            _ => Some(value),
        }
    }
}

#[derive(Debug)]
pub enum CGBFlag {
    CGBOnly = 0x80,
    SupportsCGB = 0xC0,
}

impl CGBFlag {
    pub fn from(value: u8) -> Option<CGBFlag> {
        match value {
            0x80 => Some(CGBFlag::CGBOnly),
            0xC0 => Some(CGBFlag::SupportsCGB),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Destination {
    Japanese = 0x00,
    NonJapanese = 0x01,
}

impl Destination {
    pub fn from(value: u8) -> Result<Destination, String> {
        match value {
            0x00 => Ok(Destination::Japanese),
            0x01 => Ok(Destination::NonJapanese),
            _ => Err(String::from("invalid destination code")),
        }
    }
}
