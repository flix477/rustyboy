use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};

fn context() -> (web_sys::HtmlCanvasElement, web_sys::CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

fn draw(buffer: Vec<u8>) {
    let (canvas, context) = context();
    let buffer = Clamped(buffer);

    // clear the canvas
    context.clearRect(0, 0, canvas.width(), canvas.height());

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

#[wasm_bindgen]
pub fn setup(buffer: Vec<u8>) -> GameboyJs {
    let cartridge = Cartridge::from_buffer(buffer).unwrap();
    let config = Config {
        device_type: DeviceType::GameBoy,
        debugger: None,
    };

    GameboyJs {
        gameboy: Gameboy::new(cartridge, config).unwrap()
    }
}

#[wasm_bindgen(js_name = Gameboy)]
pub struct GameboyJs {
    #[wasm_bindgen(skip)]
    pub gameboy: Gameboy
}

#[wasm_bindgen(js_class = Gameboy)]
impl GameboyJs {
    pub fn run_to_vblank(&mut self) {
        self.gameboy.run_to_vblank();
        draw(self.screen());
    }

    pub fn send_input(&mut self) {
//        self.gameboy.send_input(in)
    }

    fn screen(&self) -> Vec<u8> {
        let video = self.gameboy.hardware().video();
        video.screen().draw(video)
    }
}