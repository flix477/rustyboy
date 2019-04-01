use crate::debugger::{Debugger, DebuggerState};
use crate::processor::Processor;
use crate::bus::Bus;

pub mod breakpoint;
pub mod status;

pub enum CommandResult {
    Continue,
    Quit,
    None
}

pub struct Command<'a> {
    pub matching_values: Vec<&'a str>,
    pub callback: Box<Fn(&Vec<&str>, &mut DebuggerState, &Processor, &Bus) -> CommandResult>
}