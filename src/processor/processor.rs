use memory::Memory;
use processor::register::DualRegister;
use processor::flag_register::FlagRegister;

const CLOCK_FREQUENCY: f64 = 4.194304; // MHz

pub struct Processor {
    memory: Memory,
    registers: Registers,
    clock_frequency: f64
}

impl Processor {
    pub fn new(memory: Memory) -> Processor {
        return Processor {
            memory,
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY
        };
    }

    fn next(&self) {
        let mut pc = self.registers.program_counter;
        pc += 1;
    }

    fn ld_uu8(&mut self, value: u8, address: u8) {
        self.memory.set(address as usize, value);
    }
}

struct Registers {
    af: FlagRegister, // accumulator and flags
    bc: DualRegister,
    de: DualRegister,
    hl: DualRegister,
    stack_pointer: u16,
    program_counter: u16
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            af: FlagRegister::new(),
            bc: DualRegister::new(),
            de: DualRegister::new(),
            hl: DualRegister::new(),
            stack_pointer: 0,
            program_counter: 0
        };
    }

    pub fn af(&self) -> &FlagRegister { return &self.af; }
}