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
use crate::debugger::{Breakpoint, BreakpointCondition, DebuggerState};
use crate::gameboy::DeviceType;
use crate::processor::registers::RegisterType;
use crate::ui::run;

fn main() {
    let cartridge = Cartridge::from_file("test/cpu_instrs.gb").unwrap();
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: None,
    };

    run(config);
}
