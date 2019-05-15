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
//    let cartridge = Cartridge::from_file("test/individual/03-op sp,hl.gb").unwrap();
    let cartridge = Cartridge::from_file("tetris.gb").unwrap();
    // 01 -> passed
    // 02 -> passed
    // 03 -> failed
    // 04 -> passed
    // 05 -> passed
    // 06 -> passed
    // 07 -> passed
    // 08 -> passed
    // 09 -> failed
    // 10 -> passed
    // 11 -> stalled
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: Some(DebuggerState {
            forced_break: true,
            breakpoints: vec![
                Breakpoint {
                    line: 0xdef8,
                    conditions: Some(vec![
                        BreakpointCondition::MnemonicEquals(Mnemonic::INC),
                        BreakpointCondition::RegisterEquals(RegisterType::HL, 0xFFFF),
                        BreakpointCondition::RegisterEquals(RegisterType::SP, 0xFFFF)
                    ])
                }
            ]
        }),
    };

    run(config);
}
