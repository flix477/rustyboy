use std::fs;
mod gameboy;
mod cartridge;
mod processor;
mod util;
mod bus;
use cartridge::cartridge_metadata::CartridgeMetadata;

fn main() {
    let buffer = fs::read("tetris.gb").unwrap();
    let cm = CartridgeMetadata::from_buffer(&buffer).unwrap();
    println!("{:?}", cm);
}