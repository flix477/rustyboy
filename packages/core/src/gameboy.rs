use crate::bus::Readable;
use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::debugger::debug_info::DebugInfo;
use crate::debugger::Debugger;
use crate::hardware::{joypad::Input, Hardware};
use crate::processor::{Processor, ProcessorStepResult};
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
            let GameboyStepResult(cpu_step_result, status_mode) = self.step();
            if let Some(StatusMode::VBlank) = status_mode {
                return GameboyEvent::VBlank;
            } else if let (Some(debugger), ProcessorStepResult::NewInstruction) =
                (debugger, cpu_step_result)
            {
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

    fn step(&mut self) -> GameboyStepResult {
        GameboyStepResult(
            self.processor.step(&mut self.hardware),
            self.hardware.clock(),
        )
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

pub struct GameboyStepResult(ProcessorStepResult, Option<StatusMode>);
