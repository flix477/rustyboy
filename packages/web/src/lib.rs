use wasm_bindgen::prelude::*;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};

use crate::input::InputJs;
use crate::rendering::Renderer;
use rustyboy_core::video::color::ColorFormat;

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
        gameboy: Gameboy::new(cartridge, config),
        renderer: None,
    }
}

#[wasm_bindgen(js_name = Gameboy)]
pub struct GameboyJs {
    #[wasm_bindgen(skip)]
    pub gameboy: Gameboy,
    renderer: Option<Renderer>,
}

#[wasm_bindgen(js_class = Gameboy)]
impl GameboyJs {
    #[wasm_bindgen(js_name = runToVBlank)]
    pub fn run_to_vblank(&mut self) -> Result<(), JsValue> {
        self.gameboy.run_to_vblank();
        self.draw()
    }

    #[wasm_bindgen(js_name = sendInput)]
    pub fn send_input(&mut self, input: InputJs) {
        self.gameboy.send_input(input.into());
    }

    fn screen(&self) -> Vec<u8> {
        let screen = self.gameboy.hardware().video().screen();
        screen.buffer(ColorFormat::RGB)
    }

    fn draw(&mut self) -> Result<(), JsValue> {
        let buffer = self.screen();
        if self.renderer.is_none() {
            self.initialize_renderer()?;
        }

        if let Some(renderer) = &self.renderer {
            renderer.update(&buffer)?;
        }

        Ok(())
    }

    fn initialize_renderer(&mut self) -> Result<(), JsValue> {
        let renderer = Renderer::new()?;
        self.renderer = Some(renderer);
        Ok(())
    }
}
