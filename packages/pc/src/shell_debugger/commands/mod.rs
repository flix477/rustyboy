use super::DebuggerState;
use rustyboy_core::bus::Bus;
use rustyboy_core::debugger::debug_info::DebugInfo;

pub mod breakpoint;
pub mod continue_cmd;
pub mod quit;
pub mod status;
pub mod step_into;
pub mod step_over;

pub enum CommandResult {
    Continue,
    Quit,
}

pub trait Command {
    fn matching_value(&self) -> &[&str];
    fn execute(
        &self,
        input: &[&str],
        debugger: &mut DebuggerState,
        debug_info: &DebugInfo<'_>,
        bus: &dyn Bus,
    ) -> CommandResult;

    fn help(&self) -> String {
        self.matching_value().iter().enumerate().fold(
            String::new(),
            |acc, (idx, matching_value)| {
                if idx == 0 {
                    matching_value.to_string()
                } else {
                    format!("{}|{}", acc, matching_value)
                }
            },
        )
    }
}
