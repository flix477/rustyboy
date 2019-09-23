use crate::bus::{Readable, Writable};
use crate::sound::volume_enveloppe::VolumeEnveloppe;

#[derive(Default)]
pub struct NoiseChannel {
    pub sound_length: u8,
    pub volume_enveloppe: VolumeEnveloppe,

}

impl Readable for NoiseChannel {
    fn read(&self, address: u16) -> u8 {
        unimplemented!()
    }
}

impl Writable for NoiseChannel {
    fn write(&mut self, address: u16, value: u8) {
        unimplemented!()
    }
}