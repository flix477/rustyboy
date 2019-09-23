use crate::bus::{Readable, Writable};
use crate::sound::wave_duty_register::WaveDutyRegister;
use crate::sound::volume_enveloppe::VolumeEnveloppe;
use crate::sound::frequency_register::FrequencyRegister;

#[derive(Default)]
pub struct Channel2 {
    wave_duty_register: WaveDutyRegister,
    volume_enveloppe: VolumeEnveloppe,
    frequency_register: FrequencyRegister
}

impl Readable for Channel2 {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF16 => self.wave_duty_register.register,
            0xFF17 => self.volume_enveloppe.register,
            0xFF18 => { 0xFF }, // Write-only
            0xFF19 => self.frequency_register.high_register,
            _ => panic!("Illegal sound channel 2 read")
        }
    }
}

impl Writable for Channel2 {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF16 => self.wave_duty_register.register = value,
            0xFF17 => self.volume_enveloppe.register = value,
            0xFF18 => self.frequency_register.low_register = value,
            0xFF19 => self.frequency_register.high_register = value,
            _ => panic!("Illegal sound channel 2 write")
        }
    }
}