use glium::glutin::{Event, EventsLoop, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};
use std::process::exit;

use rustyboy_core::gameboy::Gameboy;
use rustyboy_core::video::screen::{Screen, BACKGROUND_SIZE};

use super::{create_display, Window};

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
        let buf: Vec<u8> = Screen::background(gameboy.hardware().video())
            .iter()
            .flat_map(|color| color.color.to_rgb().to_vec())
            .collect();
        let img = RawImage2d::from_raw_rgb_reversed(
            &buf,
            (BACKGROUND_SIZE.0 as u32, BACKGROUND_SIZE.1 as u32),
        );
        glium::Texture2d::new(&self.display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();

        self.events_loop.poll_events(|event| {
            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                exit(0);
            }
        });
    }
}
