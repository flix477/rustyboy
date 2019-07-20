use crate::debugger::processor_debug_info::ProcessorDebugInfo;
use crate::video::debugging::VideoDebugInformation;

#[derive(Clone)]
pub struct DebugInfo {
    pub cpu_debug_info: ProcessorDebugInfo,
    pub video_information: VideoDebugInformation,
}
