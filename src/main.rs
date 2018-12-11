mod gameboy;
mod cartridge;
mod processor;
mod util;
mod bus;
mod config;
mod video;
mod hardware;

use crate::gameboy::{Gameboy, DeviceType};
use crate::config::Config;
use crate::cartridge::Cartridge;
use crate::video::tile::Tile;
use glium::{Display, Surface};
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder, WindowEvent, Event};
use glium::texture::RawImage2d;
use std::time::Instant;
use crate::util::as_millis;
use glium::uniforms::MagnifySamplerFilter;

fn main() {
    let cartridge = Cartridge::from_file("tetris.gb").unwrap();
    println!("{:?}", cartridge.metadata());
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy
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


        gameboy.update(as_millis(delta));
        let tiles = gameboy.hardware().video().memory().tile_data();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        if tiles.len() > 0 {
            let colors = tiles[0..64].iter().map(|tile| tile.colored()).flat_map(|colors| {
                colors.iter().flat_map(|color| {
                    let color = color.to_rgb();
                    vec![color.0, color.1, color.2]
                }).collect::<Vec<u8>>()
            }).collect::<Vec<u8>>();
            let img = RawImage2d::from_raw_rgb_reversed(&colors, (64, 64));
            glium::Texture2d::new(&display, img)
                .unwrap()
                .as_surface()
                .fill(&target, MagnifySamplerFilter::Nearest);
        }

        target.finish().unwrap();
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    println!("The close button was pressed; stopping");
                    closed = true;
                },
                _ => {}
            }
        });
        last_time = now;
    }
}