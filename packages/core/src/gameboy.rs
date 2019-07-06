use crate::bus::Readable;
use crate::cartridge::Cartridge;
use crate::config::Config;
use crate::debugger::debug_info::DebugInfo;
use crate::debugger::Debugger;
use crate::hardware::{joypad::Input, Hardware};
use crate::processor::{Processor, ProcessorStepResult};
use crate::video::status_register::StatusMode;

/// This struct represents a GameBoy with all its components
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

    /// Runs the GameBoy until a VBlank interrupt occurs.
    /// Internally this is equivalent to calling `run_to_event(None)`
    pub fn run_to_vblank(&mut self) {
        loop {
            if let GameboyEvent::VBlank = self.run_to_event(None) {
                break;
            }
        }
    }

    /// Runs the GameBoy until a VBlank interrupt occurs or, if a debugger is passed to the method,
    /// until a breakpoint is hit
    pub fn run_to_event(&mut self, mut debugger: Option<&mut Debugger>) -> GameboyEvent {
        loop {
            let GameboyStepResult(cpu_step_result, status_mode) = self.step();
            if let Some(StatusMode::VBlank) = status_mode {
                return GameboyEvent::VBlank;
            } else if let (Some(debugger), ProcessorStepResult::InstructionCompleted) = (debugger.as_mut(), cpu_step_result) {
                let cpu_debug_info = self.processor.debug_info();
                if debugger.should_run(&cpu_debug_info) {
                    debugger.clean_breakpoints(&cpu_debug_info);
                    let debug_info = DebugInfo {
                        cpu_debug_info,
                        bus: self.hardware.read_all(),
                    };
                    return GameboyEvent::Debugger(debug_info);
                }
            }
        }
    }

    /// Performs a single step to all of the GameBoy's components
    fn step(&mut self) -> GameboyStepResult {
        GameboyStepResult(
            self.processor.step(&mut self.hardware),
            self.hardware.clock(),
        )
    }

    pub fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    /// Sends an button event to the GameBoy
    pub fn send_input(&mut self, input: Input) {
        self.hardware.send_input(input);
    }
}

/// Represents the type of GameBoy to emulate
pub enum DeviceType {
    GameBoy,
    GameBoyColor,
}

/// Represents the event that triggered the end of a `run_to_event` call
pub enum GameboyEvent {
    VBlank,
    Debugger(DebugInfo),
}

/// Represents the result of a single GameBoy step
pub struct GameboyStepResult(ProcessorStepResult, Option<StatusMode>);
