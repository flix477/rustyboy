use glium::glutin::EventsLoop;
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};

use rustyboy_core::gameboy::Gameboy;

use super::{create_display, Window};

pub struct BackgroundWindow {
    pub display: Display,
}

impl BackgroundWindow {
    pub fn new(events_loop: &EventsLoop) -> Self {
        Self {
            display: create_display("Rustyboy | Background", &events_loop, (256, 256)),
        }
    }
}

impl Window for BackgroundWindow {
    fn update(&self, gameboy: &Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let screen = gameboy.hardware().video().screen();

        let buf: Vec<u8> = screen
            .background(gameboy.hardware().video())
            .iter()
            .flat_map(|color| color.to_rgb().to_vec())
            .collect();

        let img = RawImage2d::from_raw_rgb_reversed(&buf, (256, 256));

        glium::Texture2d::new(&self.display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();
    }
}
