use glium::glutin::EventsLoop;
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};

use rustyboy_core::gameboy::Gameboy;
use rustyboy_core::util::drawer;
use rustyboy_core::util::drawer::Entity;
use rustyboy_core::video::color::Color;

use super::{create_display, Window};

const TILE_DATA_DIMENSIONS: (usize, usize) = (16, 24);

pub struct TileDataWindow {
    pub display: Display,
}

impl TileDataWindow {
    pub fn new(events_loop: &EventsLoop) -> TileDataWindow {
        TileDataWindow {
            display: create_display("Rustyboy | Tile Data", &events_loop, (128, 192)),
        }
    }
}

impl Window for TileDataWindow {
    fn update(&self, gameboy: &Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let mut buf = vec![Color::Black; 16 * 8 * 24 * 8];

        let tile_data = gameboy.hardware().video().memory().tile_data();
        let entities = tile_data.iter().enumerate().map(|(idx, tile)| {
            let y = idx / TILE_DATA_DIMENSIONS.0;
            let x = idx - y * TILE_DATA_DIMENSIONS.0;
            Entity {
                width: 8,
                height: 8,
                x: x * 8,
                y: y * 8,
                data: tile.colored().to_vec(),
            }
        });

        for entity in entities {
            drawer::draw_entity(entity, (16 * 8, 24 * 8), &mut buf);
        }

        let buf: Vec<u8> = buf
            .iter()
            .flat_map(|color| color.to_rgb().to_vec())
            .collect();

        let img = RawImage2d::from_raw_rgb_reversed(&buf, (16 * 8, 24 * 8));

        glium::Texture2d::new(&self.display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();
    }
}
