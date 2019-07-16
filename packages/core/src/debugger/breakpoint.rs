use crate::processor::registers::{RegisterType, Registers};

#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoint {
    pub conditions: Vec<BreakpointCondition>,
    pub one_time: bool,
}

impl Breakpoint {
    pub fn satisfied(&self, registers: &Registers) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.satisfied(registers))
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum BreakpointCondition {
    RegisterEquals(RegisterType, u16),
}

impl BreakpointCondition {
    pub fn satisfied(self, registers: &Registers) -> bool {
        match self {
            BreakpointCondition::RegisterEquals(register, value) => {
                registers.reg(register) == value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::debugger::breakpoint::{Breakpoint, BreakpointCondition};
    use crate::processor::registers::register::Register;
    use crate::processor::registers::{RegisterType, Registers};

    #[test]
    fn breakpoint_unsatisfied() {
        let mut registers = Registers::default();
        registers.program_counter.set(0);

        let breakpoint = Breakpoint {
            conditions: vec![BreakpointCondition::RegisterEquals(RegisterType::PC, 1)],
            one_time: false,
        };

        assert!(!breakpoint.satisfied(&registers));
    }

    #[test]
    fn breakpoint_satisfied() {
        let mut registers = Registers::default();
        registers.program_counter.set(0x10);

        let breakpoint = Breakpoint {
            conditions: vec![BreakpointCondition::RegisterEquals(RegisterType::PC, 0x10)],
            one_time: false,
        };

        assert!(breakpoint.satisfied(&registers));
    }

    #[test]
    fn breakpoint_partially_satisfied() {
        let mut registers = Registers::default();
        registers.program_counter.set(0x10);
        registers.hl.set(0);

        let breakpoint = Breakpoint {
            conditions: vec![
                BreakpointCondition::RegisterEquals(RegisterType::PC, 0x10),
                BreakpointCondition::RegisterEquals(RegisterType::HL, 0x10),
            ],
            one_time: false,
        };

        assert!(!breakpoint.satisfied(&registers));
    }
}
