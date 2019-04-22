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
use crate::util::drawer;
use crate::util::drawer::Entity;
use crate::video::color::Color;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Display, Surface};

fn main() {
    let cartridge = Cartridge::from_file("Tetris.gb").unwrap();
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: Some(DebuggerState {
            ..DebuggerState::default()
        }),
    };
    let mut gameboy = Gameboy::new(config).unwrap();

    let mut events_loop = EventsLoop::new();

    let main_window = MainWindow::new(&events_loop);
    let tile_window = TileDataWindow::new(&events_loop);

    let mut closed = false;
    while !closed {
        gameboy.run_to_vblank();

        main_window.update(&gameboy);
        tile_window.update(&gameboy);

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
    }
}

pub fn create_display(title: &str, events_loop: &EventsLoop) -> Display {
    let window = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(LogicalSize {
            width: 320.0,
            height: 288.0,
        });
    let ctx = ContextBuilder::new();
    Display::new(window, ctx, events_loop).unwrap()
}

trait Window {
    fn update(&self, gameboy: &Gameboy);
}

struct MainWindow {
    pub display: Display,
}

impl MainWindow {
    pub fn new(events_loop: &EventsLoop) -> MainWindow {
        MainWindow {
            display: create_display("Rustyboy", &events_loop),
        }
    }
}

impl Window for MainWindow {
    fn update(&self, gameboy: &Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let screen = gameboy.hardware().video().screen();
        let buf = screen.draw(gameboy.hardware().video());
        let img = RawImage2d::from_raw_rgb_reversed(
            &buf,
            (screen.dimensions.0 as u32, screen.dimensions.1 as u32),
        );
        glium::Texture2d::new(&self.display, img)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest);

        target.finish().unwrap();
    }
}

const TILE_DATA_DIMENSIONS: (usize, usize) = (16, 24);

struct TileDataWindow {
    pub display: Display,
}

impl TileDataWindow {
    pub fn new(events_loop: &EventsLoop) -> TileDataWindow {
        TileDataWindow {
            display: create_display("Rustyboy | Tile Data", &events_loop),
        }
    }
}

impl Window for TileDataWindow {
    fn update(&self, gameboy: &Gameboy) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let mut buf = vec![Color::Black; 16 * 8 * 24 * 8];

        let tile_data = gameboy.hardware().video().memory().tile_data();
        let entities = tile_data[4..5].iter().enumerate().map(|(idx, tile)| {
            let y = idx / TILE_DATA_DIMENSIONS.0;
            let x = idx - y * TILE_DATA_DIMENSIONS.0;
            Entity {
                width: 8,
                height: 8,
                x: (x * 8) as u8,
                y: (y * 8) as u8,
                data: tile.colored(),
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
