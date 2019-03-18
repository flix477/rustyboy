mod decoder;
mod flag_register;
mod instruction;
pub mod interrupt;
mod lr35902;
mod program_counter;
mod register;
mod registers;
mod stack_pointer;
use crate::bus::Bus;
use crate::processor::decoder::Decoder;
use crate::processor::flag_register::Flag;
use crate::processor::instruction::Prefix;
use crate::processor::lr35902::LR35902;
use crate::processor::register::Register;
use crate::processor::registers::{RegisterType, Registers};
use crate::util::bitflags::Bitflags;

const CLOCK_FREQUENCY: f64 = 4194304.0; // Hz

pub struct Processor {
    registers: Registers,
    clock_frequency: f64,
    leftover_time: f64,
    last_instruction_cycles: u8,
    stopped: bool,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: Registers::new(),
            clock_frequency: CLOCK_FREQUENCY,
            leftover_time: 0.0,
            last_instruction_cycles: 0,
            stopped: false,
        }
    }

    pub fn update<H: Bus>(&mut self, bus: &mut H, delta: f64) {
        if !self.stopped {
            self.leftover_time += delta;
            while !self.stopped
                && (self.last_instruction_cycles == 0
                    || self.leftover_time
                        >= (self.last_instruction_cycles as f64 / CLOCK_FREQUENCY))
            {
                self.leftover_time -= if self.last_instruction_cycles > 0 {
                    self.last_instruction_cycles as f64 / CLOCK_FREQUENCY
                } else {
                    self.leftover_time
                };
                self.last_instruction_cycles = self.step(bus);
            }
        } else {
            self.step(bus);
        }
    }

    pub fn step<H: Bus>(&mut self, bus: &mut H) -> u8 {
        let interrupt = bus.fetch_interrupt();
        if let Some(interrupt) = interrupt {
            self.stopped = false;
            let pc = self.registers.program_counter.get();
            self.push_stack(bus, pc);
            self.jp(interrupt.address());
            0 // lol TODO
        } else if !self.stopped {
            self.execute_next(bus, Prefix::None)
        } else {
            0
        }
    }
}

impl LR35902 for Processor {
    fn immediate<H: Bus>(&mut self, bus: &H) -> u8 {
        self.registers.program_counter.fetch(bus)
    }

    fn immediate16<H: Bus>(&mut self, bus: &H) -> u16 {
        (self.immediate(bus) as u16) | ((self.immediate(bus) as u16) << 8)
    }

    fn reg(&self, register: RegisterType) -> u16 {
        self.registers.reg(register)
    }

    fn set_reg(&mut self, register: RegisterType, value: u16) {
        self.registers.set_reg(register, value);
    }

    fn address<H: Bus>(&self, bus: &H, address: u16) -> u8 {
        bus.read(address)
    }

    fn set_address<H: Bus>(&self, bus: &mut H, address: u16, value: u8) {
        bus.write(address, value);
    }

    fn flag(&self, flag: Flag) -> bool {
        self.registers.af.flag(flag)
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        self.registers.af.set_flag(flag, value);
    }

    fn push_stack<H: Bus>(&mut self, bus: &mut H, value: u16) {
        self.registers.stack_pointer.push(bus, value);
    }

    fn pop_stack<H: Bus>(&mut self, bus: &mut H) -> u16 {
        self.registers.stack_pointer.pop(bus)
    }

    fn execute_next<H: Bus>(&mut self, bus: &mut H, prefix: Prefix) -> u8 {
        let line = self.registers.program_counter.get();
        let opcode = self.immediate(bus);
        if let Some(instruction) = Decoder::decode_opcode(opcode, prefix) {
            println!(
                "0x{:X}: {:?}, 0x{:X}",
                line,
                instruction,
                self.registers.program_counter.peek16(bus)
            );
            let cycle_count = instruction.cycle_count();
            if let Err(err) = self.execute(bus, instruction) {
                println!("Error with instruction: {:?}", err);
                panic!()
            }
            return cycle_count;
        }
        0 // i guess lol
    }

    fn halt(&mut self) {
        self.stopped = true;
    }

    fn stop(&mut self) {
        self.stopped = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::Readable;
    use crate::bus::Writable;
    use crate::processor::instruction::{AddressType as Addr, Reference, ValueType as Value};
    use crate::processor::interrupt::Interrupt;
    use crate::processor::registers::RegisterType as Reg;

    struct MockBus {
        pub memory: [u8; 65536],
        pub interrupts_enabled: bool,
    }

    impl Default for MockBus {
        fn default() -> Self {
            MockBus {
                memory: [0; 65536],
                interrupts_enabled: false,
            }
        }
    }

    impl Bus for MockBus {
        fn fetch_interrupt(&mut self) -> Option<Interrupt> {
            None
        }
        fn request_interrupt(&mut self, interrupt: Interrupt) {}
        fn toggle_interrupts(&mut self, value: bool) {
            self.interrupts_enabled = value;
        }
        fn dma_transfer(&mut self, from: u16, to: u16, size: u16) {}
    }

    impl Readable for MockBus {
        fn read(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }
    }

    impl Writable for MockBus {
        fn write(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }
    }

    fn setup() -> Processor {
        let mut cpu = Processor::new();
        cpu.set_flag(Flag::HalfCarry, false);
        cpu.set_flag(Flag::Carry, false);
        cpu.set_flag(Flag::Zero, false);
        cpu.set_flag(Flag::AddSub, false);
        cpu
    }

    #[test]
    fn ld() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        assert_eq!(cpu.reg(Reg::A), 0);
    }

    #[test]
    fn ldd_a_hl() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        // LDD A,(HL)
        // puts the value at (HL) in A, decrement HL
        cpu.ld(&mut bus, Reference::Register(Reg::HL), 0);
        bus.memory[0] = 1;

        cpu.ldd(
            &mut bus,
            Reference::Register(Reg::A),
            Value::Address(Addr::Register(Reg::HL)),
        );
        assert_eq!(std::u16::MAX, cpu.reg(Reg::HL));
        assert_eq!(1, cpu.reg(Reg::A) as u8);
    }

    #[test]
    fn ldd_hl_a() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        // LDD (HL),A
        // puts A in at memory address (HL), decrement HL
        cpu.ld(&mut bus, Reference::Register(Reg::HL), 0);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);

        cpu.ldd(
            &mut bus,
            Reference::Address(Addr::Register(Reg::HL)),
            Value::Register(Reg::A),
        );
        assert_eq!(bus.memory[0], 1);
        assert_eq!(65535, cpu.reg(Reg::HL));
    }

    #[test]
    fn ldi_hl_a() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        // LDI (HL),A
        // puts A in at memory address (HL), increment HL
        cpu.ld(&mut bus, Reference::Register(Reg::HL), 65535);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);

        cpu.ldi(
            &mut bus,
            Reference::Address(Addr::Register(Reg::HL)),
            Value::Register(Reg::A),
        );
        assert_eq!(bus.memory[65535], 1);
        assert_eq!(0, cpu.reg(Reg::HL));
    }

    #[test]
    fn ldi_a_hl() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        // LDI A,(HL)
        // puts the value at (HL) in A, increment HL
        cpu.ld(&mut bus, Reference::Register(Reg::HL), 65535);
        bus.memory[65535] = 1;

        cpu.ldi(
            &mut bus,
            Reference::Register(Reg::A),
            Value::Address(Addr::Register(Reg::HL)),
        );
        assert_eq!(0, cpu.reg(Reg::HL));
        assert_eq!(1, cpu.reg(Reg::A) as u8);
    }

    #[test]
    fn ldhl() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        // LDHL
        // Put SP + n in HL
        // n = -1
        bus.memory[cpu.reg(Reg::PC) as usize] = -1i8 as u8;
        // SP = 0
        cpu.ld(&mut bus, Reference::Register(Reg::SP), 0);

        cpu.ldhl(&mut bus);

        assert_eq!(65535, cpu.reg(Reg::HL));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        // TODO: i have no idea what value they should be lol
        //        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        //        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn push() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        // PUSH nn
        // push value nn on stack, decrement SP twice
        let original_sp = cpu.reg(Reg::SP);
        cpu.push(&mut bus, 0xeeff);
        assert_eq!(original_sp - 2, cpu.reg(Reg::SP));
        assert_eq!(0xee, bus.memory[original_sp as usize - 1]);
        assert_eq!(0xff, bus.memory[original_sp as usize - 2]);
    }

    #[test]
    fn pop() {
        let mut cpu = setup();
        let mut bus = MockBus::default();
        let test_value = 0xeeff;
        let test_reg = Reg::HL;

        // POP nn
        // pop value from stack to nn, increment SP twice

        // first we push
        cpu.push(&mut bus, test_value);

        let original_sp = cpu.reg(Reg::SP);
        cpu.pop(&mut bus, test_reg);

        assert_eq!(original_sp + 2, cpu.reg(Reg::SP));
        assert_eq!(test_value, cpu.reg(test_reg));
    }

    #[test]
    fn add_8() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.add(Reg::A, 1);

        assert_eq!(1, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn add_8_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xFF);
        cpu.add(Reg::A, 1);

        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn add_8_half_carry() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xF);
        cpu.add(Reg::A, 1);

        assert_eq!(0x10, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn add_16() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::BC), 0xF000);
        cpu.add16(Reg::BC, 0xE01);

        assert_eq!(0xFE01, cpu.reg(Reg::BC));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn add_16_half_carry() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::BC), 0xFF);
        cpu.add16(Reg::BC, 1);

        assert_eq!(0x100, cpu.reg(Reg::BC));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn add_16_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::BC), 0xFFFF);
        cpu.add16(Reg::BC, 1);

        assert_eq!(0, cpu.reg(Reg::BC));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn adc_carry_on() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xFF);
        cpu.set_flag(Flag::Carry, true);
        cpu.adc(0);

        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn adc_carry_off() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xFF);
        cpu.set_flag(Flag::Carry, false);
        cpu.adc(0);

        assert_eq!(0xFF, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn base_sub() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        let result = cpu.base_sub(1);

        assert_eq!(0, result);

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn base_sub_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        let result = cpu.base_sub(1);

        assert_eq!(0xFF, result);

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn base_sub_half_carry() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xF0);
        let result = cpu.base_sub(1);

        assert_eq!(0xEF, result);

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn sub() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sub(1);

        assert_eq!(0, cpu.reg(Reg::A));
    }

    #[test]
    fn sbc_carry_on() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sbc(0);

        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn sbc_carry_off() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, false);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sbc(0);

        assert_eq!(1, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn and() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xF);
        cpu.and(0xFF);

        assert_eq!(0xF, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn or() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xF);
        cpu.or(0xFF);

        assert_eq!(0xFF, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn xor() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xF);
        cpu.xor(0xFF);

        assert_eq!(0xF0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn inc8() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.inc(&mut bus, Reference::Register(Reg::A));

        assert_eq!(1, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn inc8_half_carry() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xF);
        cpu.inc(&mut bus, Reference::Register(Reg::A));

        assert_eq!(0x10, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn inc8_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xFF);
        cpu.inc(&mut bus, Reference::Register(Reg::A));

        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn inc16() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::BC), 0xFFFF);
        cpu.inc(&mut bus, Reference::Register(Reg::BC));

        assert_eq!(0, cpu.reg(Reg::BC));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn dec8() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.dec(&mut bus, Reference::Register(Reg::A));

        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn dec8_half_carry() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0x10);
        cpu.dec(&mut bus, Reference::Register(Reg::A));

        assert_eq!(0xF, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn dec8_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.dec(&mut bus, Reference::Register(Reg::A));

        assert_eq!(0xFF, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn dec16() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::BC), 0xFFFF);
        cpu.dec(&mut bus, Reference::Register(Reg::BC));

        assert_eq!(0xFFFE, cpu.reg(Reg::BC));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn daa() {
        // TODO
    }

    #[test]
    fn cpl() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b0101_1010);
        cpu.cpl();

        assert_eq!(0b1010_0101, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn ccf() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ccf();
        assert_eq!(true, cpu.flag(Flag::Carry));
        cpu.ccf();
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn scf() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.scf();
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn ei() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ei(&mut bus);
        assert_eq!(true, bus.interrupts_enabled);

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Carry));
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn di() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ei(&mut bus);
        cpu.di(&mut bus);
        assert_eq!(false, bus.interrupts_enabled);

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Carry));
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }
}
