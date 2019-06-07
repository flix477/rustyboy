use glium::{
    glutin::{dpi::LogicalSize, ContextBuilder, EventsLoop, WindowBuilder},
    Display,
};
use rustyboy_core::gameboy::Gameboy;

pub mod background;
pub mod screen;

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
