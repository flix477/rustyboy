use crate::processor::ProcessorStepResult;
use crate::video::status_register::StatusMode;

#[derive(Default, Copy, Clone)]
pub struct StepInput {
    pub held_buttons: u8
}

/// Represents the result of a single GameBoy step
pub struct GameboyStepResult(pub ProcessorStepResult, pub Option<StatusMode>);
