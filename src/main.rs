mod bus;
mod cartridge;
mod config;
mod debugger;
mod gameboy;
mod hardware;
mod processor;
mod tests;
mod util;
mod video;

use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::debugger::DebuggerState;
use crate::gameboy::{DeviceType, Gameboy};
use crate::util::as_millis;
use crate::video::color::Color;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};
use std::time::Instant;

fn main() {
    let cartridge = Cartridge::from_file("tetris.gb").unwrap();
    println!("{:?}", cartridge.metadata());
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: Some(DebuggerState {
            forced_break: true,
            ..DebuggerState::default()
        }),
    };
    let mut gameboy = Gameboy::new(config).unwrap();
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new();
    let context = ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;
    let mut last_time = Instant::now();
    while !closed {
        let now = Instant::now();
        let delta = now.duration_since(last_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        gameboy.run_to_vblank();

        //        let screen = gameboy.hardware().video().screen();
        //        let buf = screen.draw(gameboy.hardware().video());

        let tile_data = gameboy.hardware().video().memory().tile_data();
        let colors = tile_data
            .iter()
            .flat_map(|tile| tile.colored().to_vec())
            .collect::<Vec<Color>>();
        let buf = colors
            .iter()
            .flat_map(|color| color.to_rgb().to_vec())
            .collect::<Vec<u8>>();

        let img = RawImage2d::from_raw_rgb_reversed(&buf, (16 * 8, 24 * 8));
        glium::Texture2d::new(&display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                closed = true;
            }
            _ => {}
        });
        last_time = now;
    }
}
