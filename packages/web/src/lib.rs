use wasm_bindgen::prelude::*;

use crate::gameboy::GameboyJs;
use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};

pub mod debugger;
pub mod gameboy;
pub mod input;
pub mod rendering;

#[wasm_bindgen]
pub fn setup(buffer: Vec<u8>) -> GameboyJs {
    let cartridge = Cartridge::from_buffer(buffer).unwrap();
    let config = Config {
        device_type: DeviceType::GameBoy,
        debugger: None,
    };

    GameboyJs {
        gameboy: Gameboy::new(cartridge, &config),
        renderer: None,
    }
}
