use crate::bus::Readable;
use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::debugger::debug_info::DebugInfo;
use crate::debugger::Debugger;
use crate::hardware::{joypad::Input, Hardware};
use crate::processor::Processor;
use crate::video::status_register::StatusMode;

pub struct Gameboy {
    processor: Processor,
    hardware: Hardware,
}

impl Gameboy {
    pub fn new(cartridge: Cartridge, _config: &Config) -> Gameboy {
        Gameboy {
            processor: Processor::new(),
            hardware: Hardware::new(cartridge),
        }
    }

    pub fn run_to_vblank(&mut self) {
        loop {
            if let GameboyEvent::VBlank = self.run_to_event(None) {
                break;
            }
        }
    }

    pub fn run_to_event(&mut self, debugger: Option<&Debugger>) -> GameboyEvent {
        loop {
            if let Some(StatusMode::VBlank) = self.step() {
                return GameboyEvent::VBlank;
            } else if let Some(debugger) = debugger {
                let cpu_debug_info = self.processor.debug_info();
                if debugger.should_run(&cpu_debug_info) {
                    let debug_info = DebugInfo {
                        cpu_debug_info,
                        bus: self.hardware.read_all(),
                    };
                    return GameboyEvent::Debugger(debug_info);
                }
            }
        }
    }

    fn step(&mut self) -> Option<StatusMode> {
        self.processor.step(&mut self.hardware);
        self.hardware.clock()
    }

    pub fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    pub fn send_input(&mut self, input: Input) {
        self.hardware.send_input(input);
    }
}

pub enum DeviceType {
    GameBoy,
    GameBoyColor,
}

pub enum GameboyEvent {
    VBlank,
    Debugger(DebugInfo),
}
