use glium::glutin::{Event, EventsLoop, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};

use rustyboy_core::gameboy::Gameboy;
use rustyboy_core::video::screen::SCREEN_SIZE;

use super::{create_display, Window};
use crate::keymap::keymap;
use crate::window::UpdateResult;

pub struct MainWindow {
    display: Display,
    events_loop: EventsLoop,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let events_loop = EventsLoop::new();

        MainWindow {
            display: create_display("Rustyboy", SCREEN_SIZE, &events_loop),
            events_loop,
        }
    }
}

impl Window for MainWindow {
    fn update(&mut self, gameboy: &mut Gameboy) -> UpdateResult {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let buf = gameboy.hardware().video.screen().buffer.rgb();
        let img =
            RawImage2d::from_raw_rgb_reversed(&buf, (SCREEN_SIZE.0 as u32, SCREEN_SIZE.1 as u32));
        glium::Texture2d::new(&self.display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();

        let mut close = false;
        self.events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                close = true;
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
        });

        if close {
            UpdateResult::Close
        } else {
            UpdateResult::Continue
        }
    }
}
