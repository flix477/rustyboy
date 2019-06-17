use glium::glutin::{Event, EventsLoop, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};
use std::process::exit;

use rustyboy_core::gameboy::Gameboy;

use super::{create_display, Window};
use rustyboy_core::util::drawer::{draw_entity, DrawnColor, Entity};

const SIZE: (usize, usize) = (128, 192);
const RELATIVE_SIZE: (usize, usize) = (SIZE.0 / 8, SIZE.1 / 8);

pub struct TileDataWindow {
    display: Display,
    events_loop: EventsLoop,
}

impl TileDataWindow {
    pub fn new() -> TileDataWindow {
        let events_loop = EventsLoop::new();

        TileDataWindow {
            display: create_display("Rustyboy | Tile data", SIZE, &events_loop),
            events_loop,
        }
    }
}

impl Window for TileDataWindow {
    fn update(&mut self, gameboy: &mut Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let video = gameboy.hardware().video();
        let tile_data = video.memory().tile_data();
        let palette = video.bg_palette();
        let mut buf = vec![DrawnColor::default(); SIZE.0 * SIZE.1];
        tile_data.iter().enumerate().for_each(|(idx, tile)| {
            let y = (idx - idx % RELATIVE_SIZE.0) / RELATIVE_SIZE.0;
            let x = idx - y * RELATIVE_SIZE.0;
            let entity = Entity::from_tile(tile, x * 8, y * 8);
            draw_entity(entity, SIZE, &mut buf, palette);
        });

        let buf = buf
            .iter()
            .flat_map(|color| color.color.to_rgb().to_vec())
            .collect::<Vec<u8>>();

        let img = RawImage2d::from_raw_rgb_reversed(&buf, (SIZE.0 as u32, SIZE.1 as u32));
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
