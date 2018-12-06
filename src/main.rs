extern crate glium;

mod gameboy;
mod cartridge;
mod processor;
mod util;
mod bus;
mod config;
mod video;
mod hardware;

use gameboy::{Gameboy, DeviceType};
use config::Config;
use cartridge::Cartridge;
//use glium::{glutin, Display, Surface};
//use glutin::{EventsLoop, WindowBuilder, ContextBuilder, ControlFlow, WindowEvent, Event};

fn main() {
    let cartridge = Cartridge::from_file("tetris.gb").unwrap();
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoyColor
    };
    let mut gameboy = Gameboy::new(config).unwrap();
    gameboy.start();
//    let mut events_loop = EventsLoop::new();
//    let window = WindowBuilder::new();
//    let context = ContextBuilder::new();
//    let display = Display::new(window, context, &events_loop).unwrap();
//
//    let mut closed = false;
//    while !closed {
//        let mut target = display.draw();
//        target.clear_color(0.0, 0.0, 1.0, 1.0);
//        target.finish().unwrap();
//
//        events_loop.poll_events(|event| {
//            match event {
//                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
//                    println!("The close button was pressed; stopping");
//                    closed = true;
//                },
//                _ => {}
//            }
//        });
//    }
}