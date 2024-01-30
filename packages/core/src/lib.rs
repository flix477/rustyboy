uniffi::include_scaffolding!("bindings");

use std::sync::Mutex;
use crate::hardware::joypad::Button;
use crate::cartridge::Cartridge;
use crate::gameboy::Gameboy;
use crate::cartridge::cartridge_metadata_error::CartridgeMetadataError;
use crate::util::savestate::LoadSavestateError;
use crate::step::StepInput;

pub mod bus;
pub mod step;
pub mod cartridge;
pub mod config;
pub mod debugger;
pub mod gameboy;
pub mod hardware;
pub mod processor;
pub mod util;
pub mod video;

pub struct RustyboyGameboy {
    gameboy: Mutex<Gameboy>
}

impl RustyboyGameboy {
    pub fn new(buffer: Vec<u8>) -> Result<Self, CartridgeMetadataError> {
        Ok(Self {
            gameboy: Mutex::new(Gameboy::new(Cartridge::from_buffer(buffer)?))
        })
    }

    pub fn reset(&self) {
        self.gameboy.lock().unwrap().reset();
    }

    pub fn run_to_vblank(&self, input: StepInput) -> Vec<u8> {
        let mut gb = self.gameboy.lock().unwrap();
        gb.run_to_vblank(input);
        gb.hardware().video.screen().buffer.rgba().to_vec()
    }

    pub fn dump_savestate(&self) -> Vec<u8> {
        self.gameboy.lock().unwrap().dump_savestate()
    }

    pub fn load_savestate(&self, buffer: Vec<u8>) -> Result<(), LoadSavestateError> {
        self.gameboy.lock().unwrap().load_savestate(buffer)
    }
}
