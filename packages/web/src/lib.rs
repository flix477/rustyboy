use wasm_bindgen::prelude::*;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};
use rustyboy_core::video::color::ColorFormat;
use rustyboy_core::video::screen::{Screen, SCREEN_SIZE};

use self::input::InputJs;

pub mod input;

#[wasm_bindgen]
pub fn setup(buffer: Vec<u8>) -> GameboyJs {
    let cartridge = Cartridge::from_buffer(buffer).unwrap();
    let config = Config {
        device_type: DeviceType::GameBoy,
        debugger: None,
    };

    GameboyJs {
        gameboy: Gameboy::new(cartridge, config).unwrap(),
    }
}

#[wasm_bindgen(js_name = Gameboy)]
pub struct GameboyJs {
    #[wasm_bindgen(skip)]
    pub gameboy: Gameboy,
}

#[wasm_bindgen(js_class = Gameboy)]
impl GameboyJs {
    #[wasm_bindgen(js_name = runToVBlank)]
    pub fn run_to_vblank(&mut self) -> Vec<u8> {
        self.gameboy.run_to_vblank();
        self.screen()
    }

    #[wasm_bindgen(js_name = sendInput)]
    pub fn send_input(&mut self, input: InputJs) {
        self.gameboy.send_input(input.into());
    }

    fn screen(&self) -> Vec<u8> {
        let video = self.gameboy.hardware().video();
        Screen::draw_with_options(video, ColorFormat::RGBA)
    }
}

#[wasm_bindgen(js_name = getScreenWidth)]
pub fn screen_width() -> usize {
    SCREEN_SIZE.0
}

#[wasm_bindgen(js_name = getScreenHeight)]
pub fn screen_height() -> usize {
    SCREEN_SIZE.1
}