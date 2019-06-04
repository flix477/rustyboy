#[macro_use]
extern crate clap;

mod app;
mod keymap;
mod window;

fn main() {
    app::run();
}
