use glium::Display;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowEvent, WindowBuilder};
use glium::glutin::dpi::LogicalSize;

use crate::gameboy::Gameboy;
use crate::config::Config;

use self::screen::MainWindow;
use self::tile_data::TileDataWindow;

pub mod screen;
pub mod tile_data;

pub trait Window {
    fn update(&self, gameboy: &Gameboy);
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

pub fn run(config: Config) {
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