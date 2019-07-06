use crate::debugger::debug_info::ProcessorDebugInfo;
use crate::processor::registers::RegisterType;

#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoint {
    pub conditions: Vec<BreakpointCondition>,
    pub one_time: bool,
}

impl Breakpoint {
    pub fn satisfied(&self, debug_info: &ProcessorDebugInfo) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.satisfied(debug_info))
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum BreakpointCondition {
    RegisterEquals(RegisterType, u16),
}

impl BreakpointCondition {
    pub fn satisfied(self, debug_info: &ProcessorDebugInfo) -> bool {
        match self {
            BreakpointCondition::RegisterEquals(register, value) => {
                debug_info.registers.reg(register) == value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::debugger::breakpoint::{Breakpoint, BreakpointCondition};
    use crate::processor::registers::register::Register;
    use crate::processor::registers::{RegisterType, Registers};
    use crate::tests::util::mock_debug_info;

    #[test]
    fn breakpoint_unsatisfied() {
        let mut registers = Registers::default();
        registers.program_counter.set(0);

        let debug_info = mock_debug_info(registers, vec![]);
        let breakpoint = Breakpoint {
            conditions: vec![BreakpointCondition::RegisterEquals(RegisterType::PC, 1)],
            one_time: false,
        };

        assert!(!breakpoint.satisfied(&debug_info.cpu_debug_info));
    }

    #[test]
    fn breakpoint_satisfied() {
        let mut registers = Registers::default();
        registers.program_counter.set(0x10);

        let debug_info = mock_debug_info(registers, vec![]);
        let breakpoint = Breakpoint {
            conditions: vec![BreakpointCondition::RegisterEquals(RegisterType::PC, 0x10)],
            one_time: false,
        };

        assert!(breakpoint.satisfied(&debug_info.cpu_debug_info));
    }

    #[test]
    fn breakpoint_partially_satisfied() {
        let mut registers = Registers::default();
        registers.program_counter.set(0x10);
        registers.hl.set(0);

        let debug_info = mock_debug_info(registers, vec![]);
        let breakpoint = Breakpoint {
            conditions: vec![
                BreakpointCondition::RegisterEquals(RegisterType::PC, 0x10),
                BreakpointCondition::RegisterEquals(RegisterType::HL, 0x10),
            ],
            one_time: false,
        };

        assert!(!breakpoint.satisfied(&debug_info.cpu_debug_info));
    }
}
