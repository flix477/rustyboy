pub enum LoadSavestateError {
    InvalidSavestate,
}

pub trait Savestate {
    fn dump_savestate(&self, buffer: &mut Vec<u8>);
    fn load_savestate<'a>(
        &mut self,
        buffer: &mut std::slice::Iter<u8>,
    ) -> Result<(), LoadSavestateError>;
}

pub fn read_savestate_byte<'a>(
    buffer: &mut impl Iterator<Item = &'a u8>,
) -> Result<u8, LoadSavestateError> {
    buffer
        .next()
        .cloned()
        .ok_or(LoadSavestateError::InvalidSavestate)
}

pub fn read_savestate_bool<'a>(
    buffer: &mut impl Iterator<Item = &'a u8>,
) -> Result<bool, LoadSavestateError> {
    Ok(read_savestate_byte(buffer)? != 0)
}

pub fn read_savestate_u16<'a>(
    buffer: &mut impl Iterator<Item = &'a u8>,
) -> Result<u16, LoadSavestateError> {
    Ok(u16::from(read_savestate_byte(buffer)?) | (u16::from(read_savestate_byte(buffer)?) << 8))
}

pub fn write_savestate_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.push(value as u8);
    buffer.push((value >> 8) as u8);
}
