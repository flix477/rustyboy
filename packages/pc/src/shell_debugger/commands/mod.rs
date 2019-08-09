use super::Debugger;
use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::debugger::DebuggerActionResult;

pub mod breakpoint;
pub mod continue_cmd;
pub mod quit;
pub mod status;
pub mod step_into;

pub enum CommandResult {
    Continue,
    Quit,
}

impl From<DebuggerActionResult> for CommandResult {
    fn from(result: DebuggerActionResult) -> Self {
        match result {
            DebuggerActionResult::Resume => CommandResult::Quit,
            DebuggerActionResult::None => CommandResult::Continue,
        }
    }
}

pub trait Command {
    fn matching_value(&self) -> &[&str];
    fn execute(
        &self,
        input: &[&str],
        debugger: &mut Debugger,
        debug_info: &DebugInfo,
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
