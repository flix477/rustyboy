use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::texture::RawImage2d;
use glium::Display;

use crate::config::Config;
use crate::gameboy::Gameboy;

use self::background::BackgroundWindow;

pub mod background;
pub mod screen;
pub mod tile_data;

pub trait Window {
    fn update(&self, gameboy: &Gameboy);
}

pub fn create_display(
    title: &str,
    events_loop: &EventsLoop,
    dimensions: (usize, usize),
) -> Display {
    let window = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(LogicalSize {
            width: dimensions.0 as f64,
            height: dimensions.1 as f64,
        });
    let ctx = ContextBuilder::new();
    Display::new(window, ctx, events_loop).unwrap()
}

pub fn to_raw_image(buf: &[u8], dimensions: (usize, usize)) -> RawImage2d<u8> {
    RawImage2d::from_raw_rgb_reversed(&buf, (16 * 8, 24 * 8))
}

pub fn run(config: Config) {
    let mut gameboy = Gameboy::new(config).unwrap();

    let mut events_loop = EventsLoop::new();

    //    let main_window = MainWindow::new(&events_loop);
    //    let tile_window = TileDataWindow::new(&events_loop);
    let background_window = BackgroundWindow::new(&events_loop);

    let mut closed = false;
    while !closed {
        gameboy.run_to_vblank();

        //        main_window.update(&gameboy);
        //        tile_window.update(&gameboy);
        background_window.update(&gameboy);

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
