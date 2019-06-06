use self::debug_info::DebugInfo;
use crate::bus::Bus;

pub mod debug_info;

pub trait Debugger {
    fn should_run(&self, debug_info: &DebugInfo) -> bool;
    fn run(&mut self, debug_info: DebugInfo, bus: &Bus);
}
