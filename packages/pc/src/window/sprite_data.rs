use glium::glutin::{Event, EventsLoop, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};
use std::process::exit;

use rustyboy_core::gameboy::Gameboy;

use super::{create_display, Window};
use rustyboy_core::util::drawer::{draw_entity, DrawnColor, Entity};
use rustyboy_core::video::screen::Sprite;

const RELATIVE_SIZE: (usize, usize) = (8, 5);
const SIZE: (usize, usize) = (RELATIVE_SIZE.0 * 8, RELATIVE_SIZE.1 * 8);

pub struct SpriteDataWindow {
    display: Display,
    events_loop: EventsLoop,
}

impl SpriteDataWindow {
    pub fn new() -> SpriteDataWindow {
        let events_loop = EventsLoop::new();

        SpriteDataWindow {
            display: create_display("Rustyboy | Tile data", SIZE, &events_loop),
            events_loop,
        }
    }
}

impl Window for SpriteDataWindow {
    fn update(&mut self, gameboy: &mut Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let video = gameboy.hardware().video();
        let sprite_data = video.memory().oam().entries();
        let tile_data = video.memory().tile_data();

        let mut buf = vec![DrawnColor::default(); SIZE.0 * SIZE.1];
        sprite_data
            .iter()
            .enumerate()
            .filter(|(_, sprite)| sprite.visible())
            .map(|(idx, sprite)| {
                (
                    idx,
                    Sprite {
                        tiles: vec![tile_data[sprite.tile_number as usize]],
                        attributes: *sprite,
                    },
                )
            })
            .for_each(|(idx, sprite)| {
                let y = (idx - idx % RELATIVE_SIZE.0) / RELATIVE_SIZE.0;
                let x = idx - y * RELATIVE_SIZE.0;
                let entity = Entity::from_tile(&sprite.tiles[0], x * 8, y * 8);
                let palette = video.obj_palette(sprite.attributes.obj_palette_number());
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
