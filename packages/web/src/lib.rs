use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use js_sys::Promise;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap, ImageData};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use futures::Future;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};
use rustyboy_core::video::color::ColorFormat;
use rustyboy_core::video::screen::{Screen, SCREEN_SIZE};

use self::input::InputJs;

pub mod input;

fn context() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

fn draw(buffer: &mut Vec<u8>) -> Result<Promise, JsValue> {
    let (canvas, context) = context();

    let window = web_sys::window().unwrap();
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(buffer.as_mut_slice()),
        SCREEN_SIZE.0 as u32,
        SCREEN_SIZE.1 as u32,
    )?;
    let test = Closure::wrap(Box::new(move |image_bitmap| {
//        context.draw_image
    }) as Box<dyn FnMut()>);
    let promise = window.create_image_bitmap_with_image_data(&image_data)?
        .then(test.as_ref().unchecked_ref());

    Ok(promise)
}

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
    pub fn run_to_vblank(&mut self) -> Result<Promise, JsValue> {
        self.gameboy.run_to_vblank();
        let mut buffer = self.screen();
        draw(&mut buffer)
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
