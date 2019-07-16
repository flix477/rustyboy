use crate::debugger::breakpoint::{Breakpoint, BreakpointCondition};
use crate::debugger::debug_info::{DebugInfo, ProcessorDebugInfo};
use crate::debugger::debug_operand_parser::DebugOperandParser;
use crate::debugger::{Debugger, DebuggerActionResult};
use crate::processor::registers::register::Register;
use crate::processor::registers::RegisterType;

pub fn run(debugger: &mut Debugger, debug_info: &DebugInfo) -> DebuggerActionResult {
    debugger.breakpoints.push(Breakpoint {
        one_time: true,
        conditions: vec![BreakpointCondition::RegisterEquals(
            RegisterType::PC,
            next_instruction_address(&debug_info.cpu_debug_info),
        )],
    });

    DebuggerActionResult::Resume
}

fn next_instruction_address(debug_info: &ProcessorDebugInfo) -> u16 {
    let line = debug_info.current_line();
    let mut parser = DebugOperandParser::new(line, debug_info);
    debug_info.parse_instruction_with_parser(&mut parser);
    parser.program_counter().get()
}

#[cfg(test)]
mod tests {
    use crate::debugger::commands::step_over::next_instruction_address;
    use crate::processor::registers::register::Register;
    use crate::processor::registers::Registers;
    use crate::tests::util::mock_debug_info;

    #[test]
    fn gets_next_address_no_operands() {
        let mut registers = Registers::default();
        registers.program_counter.set(0);

        let debug_info = mock_debug_info(
            registers,
            vec![
                0, // NOP
                0, // NOP
            ],
        );

        assert_eq!(1, next_instruction_address(&debug_info));
    }

    #[test]
    fn gets_next_address_with_operands() {
        let mut registers = Registers::default();
        registers.program_counter.set(1);

        let debug_info = mock_debug_info(
            registers,
            vec![
                0, 0x01, // LD BC,nn
                0, 0, 0, // NOP
            ],
        );

        assert_eq!(4, next_instruction_address(&debug_info));
    }
}
