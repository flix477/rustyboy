use crate::bus::Readable;
use crate::cartridge::Cartridge;
use crate::debugger::debug_info::DebugInfo;
use crate::debugger::processor_debug_info::ProcessorDebugInfo;
use crate::debugger::Debugger;
use crate::hardware::Hardware;
use crate::processor::{Processor, ProcessorStepResult};
use crate::util::savestate::{LoadSavestateError, Savestate};
use crate::video::screen::BUFFER_SIZE;
use crate::video::status_register::StatusMode;
use crate::step::{StepInput, GameboyStepResult};

/// This struct represents a GameBoy with all its components
pub struct Gameboy {
    processor: Processor,
    hardware: Hardware,
}

impl Gameboy {
    pub fn new(cartridge: Cartridge) -> Gameboy {
        Gameboy {
            processor: Processor::new(),
            hardware: Hardware::new(cartridge),
        }
    }

    /// Resets the Gameboy to its initial state
    pub fn reset(&mut self) {
        self.processor = Processor::new();
        self.hardware.reset();
    }

    /// Runs the GameBoy until a VBlank interrupt occurs.
    /// Internally this is equivalent to calling `run_to_event(None)`
    pub fn run_to_vblank(&mut self, input: StepInput) {
        loop {
            if let GameboyStepResult(_, Some(StatusMode::VBlank)) = self.step(input) {
                break;
            }
        }
    }

    /// Runs the GameBoy until a VBlank interrupt occurs or, if a debugger is passed to the method,
    /// until a breakpoint is hit
    pub fn run_to_event(&mut self, input: StepInput, mut debugger: Option<&mut Debugger>) -> GameboyEvent {
        loop {
            let GameboyStepResult(cpu_step_result, status_mode) = self.step(input);
            if let Some(StatusMode::VBlank) = status_mode {
                return GameboyEvent::VBlank;
            } else if let (Some(debugger), ProcessorStepResult::InstructionCompleted) =
                (debugger.as_mut(), cpu_step_result)
            {
                let registers = self.processor.registers;
                if debugger.should_run(&registers) {
                    let cpu_debug_info = ProcessorDebugInfo {
                        registers,
                        bus: self.hardware.read_all(),
                    };
                    debugger.clean_breakpoints(&cpu_debug_info);
                    let debug_info = DebugInfo {
                        cpu_debug_info,
                        video_information: self.hardware.video.debug_information(),
                    };
                    return GameboyEvent::Debugger(Box::new(debug_info));
                }
            }
        }
    }

    /// Performs a single step to all of the GameBoy's components
    fn step(&mut self, input: StepInput) -> GameboyStepResult {
        GameboyStepResult(
            self.processor.step(&mut self.hardware),
            self.hardware.step(input),
        )
    }

    pub fn hardware(&self) -> &Hardware {
        &self.hardware
    }

    pub fn dump_savestate(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.processor.dump_savestate(&mut buffer);
        self.hardware.dump_savestate(&mut buffer);
        buffer
    }

    pub fn load_savestate(&mut self, buffer: Vec<u8>) -> Result<(), LoadSavestateError> {
        let mut iter = buffer.iter();
        self.processor.load_savestate(&mut iter)?;
        self.hardware.load_savestate(&mut iter)?;
        Ok(())
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
    Debugger(Box<DebugInfo>),
}

impl Iterator for Gameboy {
    type Item = [u8; BUFFER_SIZE * 3];

    fn next(&mut self) -> Option<Self::Item> {
        self.run_to_vblank(StepInput::default());
        Some(self.hardware().video.screen().buffer.rgb())
    }
}
