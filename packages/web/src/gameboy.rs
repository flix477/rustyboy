use wasm_bindgen::prelude::*;

use crate::debugger::{debug_info::DebugInfoJs, DebuggerJs};
use crate::input::InputJs;
use crate::rendering::Renderer;
use rustyboy_core::gameboy::{Gameboy, GameboyEvent};
use rustyboy_core::video::color::ColorFormat;
use crate::log;

#[wasm_bindgen(js_name = Gameboy)]
pub struct GameboyJs {
    #[wasm_bindgen(skip)]
    pub gameboy: Gameboy,
    #[wasm_bindgen(skip)]
    pub renderer: Option<Renderer>,
}

#[wasm_bindgen(js_class = Gameboy)]
impl GameboyJs {
    #[wasm_bindgen(js_name = runToVBlank)]
    pub fn run_to_vblank(&mut self) -> Result<(), JsValue> {
        self.gameboy.run_to_vblank();
        self.draw()
    }

    #[wasm_bindgen(js_name = runToEvent)]
    pub fn run_to_event(
        &mut self,
        debugger_ref: &mut DebuggerJs,
    ) -> Result<Option<DebugInfoJs>, JsValue> {
        let event = self.gameboy.run_to_event(Some(&mut debugger_ref.debugger));
        self.draw()?;
        log(debugger_ref.debugger.breakpoints.len().to_string().as_str());
        if let GameboyEvent::Debugger(debug_info) = event {
            return Ok(Some(DebugInfoJs { debug_info }));
        }

        Ok(None)
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
