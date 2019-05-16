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
use crate::processor::instruction::Mnemonic;

fn main() {
//    let cartridge = Cartridge::from_file("test/individual/11-op a,(hl).gb").unwrap();
    let cartridge = Cartridge::from_file("tetris.gb").unwrap();
    // 01 -> passed
    // 02 -> passed
    // 03 -> failed (ALL TESTS -> something's wrong in the setup)
    // 04 -> passed
    // 05 -> passed
    // 06 -> passed
    // 07 -> passed
    // 08 -> passed
    // 09 -> passed
    // 10 -> passed
    // 11 -> stalled
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: Some(DebuggerState {
            forced_break: false,
            breakpoints: vec![
//                Breakpoint {
//                    line: 0xdef9,
//                    conditions: Some(vec![
//                        BreakpointCondition::MnemonicEquals(Mnemonic::SLA),
//                    ])
//                }
            ]
        }),
    };

    run(config);
}
