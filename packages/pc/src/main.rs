#[macro_use]
extern crate clap;

mod app;
mod keymap;
mod shell_debugger;
mod util;
mod window;

fn main() {
    app::run();
}
