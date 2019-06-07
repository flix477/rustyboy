use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};

fn context() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn happy() {
    let context = context();

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
pub fn run(buffer: Vec<u8>) {
    let cartridge = Cartridge::from_buffer(buffer).unwrap();
    let config = Config {
        device_type: DeviceType::GameBoy,
        debugger: None,
    };
    start_emulation(cartridge, config);
}

fn start_emulation(cartridge: Cartridge, config: Config) {
    let mut gameboy = Gameboy::new(cartridge, config).unwrap();
    happy();
}
