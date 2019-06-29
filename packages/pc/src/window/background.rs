use glium::glutin::{Event, EventsLoop, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};
use std::process::exit;

use rustyboy_core::gameboy::Gameboy;
use rustyboy_core::video::debugging::background_map_buffer;
use rustyboy_core::video::screen::BACKGROUND_SIZE;

use super::{create_display, Window};
use crate::keymap::keymap;
use rustyboy_core::video::color::ColorFormat;

pub struct BackgroundWindow {
    display: Display,
    events_loop: EventsLoop,
}

impl BackgroundWindow {
    pub fn new() -> BackgroundWindow {
        let events_loop = EventsLoop::new();

        BackgroundWindow {
            display: create_display("Rustyboy | Background", BACKGROUND_SIZE, &events_loop),
            events_loop,
        }
    }
}

impl Window for BackgroundWindow {
    fn update(&mut self, gameboy: &mut Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let video = gameboy.hardware().video().video_information();
        let buffer = background_map_buffer(video.control.bg_map(), &video, ColorFormat::RGB);

        let img = RawImage2d::from_raw_rgb_reversed(
            &buffer,
            (BACKGROUND_SIZE.0 as u32, BACKGROUND_SIZE.1 as u32),
        );
        glium::Texture2d::new(&self.display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();

        self.events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                exit(0);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                let input = keymap(input);
                if let Some(input) = input {
                    gameboy.send_input(input);
                }
            }
            _ => {}
        })
    }
}
