use crate::debugger::breakpoint::{Breakpoint, BreakpointCondition};
use crate::debugger::debug_info::DebugInfo;
use crate::debugger::debug_operand_parser::DebugOperandParser;
use crate::debugger::{Debugger, DebuggerActionResult};
use crate::processor::registers::register::Register;
use crate::processor::registers::RegisterType;

pub fn run(debugger: &mut Debugger, debug_info: &DebugInfo) -> DebuggerActionResult {
    let line = debug_info.current_line();
    let mut parser = DebugOperandParser::new(line, debug_info);
    debug_info.parse_instruction_with_parser(&mut parser);
    let next_address = parser.program_counter().get();

    debugger.breakpoints.push(Breakpoint {
        one_time: true,
        conditions: vec![BreakpointCondition::RegisterEquals(
            RegisterType::PC,
            next_address,
        )],
    });

    DebuggerActionResult::Resume
}
