use std::fs;
mod memory;
mod cartridge;
mod processor;
mod util;
use cartridge::cartridge_metadata::CartridgeMetadata;

fn main() {
    let buffer = fs::read("pokemonb.gb").unwrap();
    let cm = CartridgeMetadata::from_buffer(&buffer).unwrap();
    println!("{:?}", cm);
}