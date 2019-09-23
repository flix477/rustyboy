use crate::bus::{Readable, Writable};
use crate::sound::channels::wave_channel::WaveChannel;
use crate::sound::channels::noise_channel::NoiseChannel;
use crate::sound::channels::channel1::Channel1;
use crate::sound::channels::channel2::Channel2;
use crate::sound::output_register::OutputRegister;

mod channels;
mod frequency_register;
mod output_register;
mod sweep_register;
mod volume_enveloppe;
mod wave_duty_register;

#[derive(Default)]
pub struct Sound {
    channel1: Channel1,
    channel2: Channel2,
    wave_channel: WaveChannel, // wave output
    noise_channel: NoiseChannel,
    output_register: OutputRegister,
    sound_enabled: bool
}

impl Sound {
    pub fn fill_buffer(&self, outputs: [&mut [u8]; 2]) {

    }
}

impl Readable for Sound {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.channel1.read(address),
            0xFF16..=0xFF19 => self.channel2.read(address),
            0xFF1A..=0xFF1E
            | 0xFF30..=0xFF3F => self.wave_channel.read(address),
            0xFF20..=0xFF23 => self.noise_channel.read(address),
            0xFF25 => self.output_register.register,
            _ => panic!("Invalid APU read")
        }
    }
}

impl Writable for Sound {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF10..=0xFF14 => self.channel1.write(address, value),
            0xFF16..=0xFF19 => self.channel2.write(address, value),
            0xFF1A..=0xFF1E
            | 0xFF30..=0xFF3F => self.wave_channel.write(address, value),
            0xFF20..=0xFF23 => self.noise_channel.write(address, value),
            0xFF25 => self.output_register.register = value,
            _ => panic!("Invalid APU read")
        }
    }
}