mod bus;
mod cartridge;
mod config;
mod debugger;
mod gameboy;
mod hardware;
mod processor;
mod tests;
mod ui;
mod util;
mod video;

use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::debugger::DebuggerState;
use crate::gameboy::DeviceType;
use crate::ui::{run, Window};

fn main() {
    let cartridge = Cartridge::from_file("Tetris.gb").unwrap();
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: Some(DebuggerState {
            forced_break: true,
            ..DebuggerState::default()
        }),
    };

    run(config);
}