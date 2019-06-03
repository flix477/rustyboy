use crate::processor::flag_register::Flag;
use crate::processor::instruction::{AddressType as Addr, Reference, ValueType as Value};
use crate::processor::lr35902::LR35902;
use crate::processor::registers::RegisterType as Reg;
use crate::processor::Processor;
use crate::tests::util::mock_bus::MockBus;

fn setup() -> Processor {
    let mut cpu = Processor::new(None);
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

#[cfg(test)]
mod ldd {
    use super::*;

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
}

#[cfg(test)]
mod ldi {
    use super::*;

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

#[cfg(test)]
mod add {
    use super::*;

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

        cpu.ld(&mut bus, Reference::Register(Reg::HL), 0x2600);
        cpu.add16(Reg::HL, 0x2600);

        assert_eq!(0x4C00, cpu.reg(Reg::HL));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
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
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod adc {
    use super::*;

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
    fn adc_misc() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::AF), 0xF0);
        cpu.adc(0xFF);

        assert_eq!(0xB0, cpu.reg(Reg::AF));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod base_sub {
    use super::*;

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
}

#[test]
fn sub() {
    let mut cpu = setup();
    let mut bus = MockBus::default();

    cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
    cpu.sub(1);

    assert_eq!(0, cpu.reg(Reg::A));
}

#[cfg(test)]
mod sbc {
    use super::*;

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
    fn sbc_misc() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.sbc(0x80);

        assert_eq!(0x7F, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn sbc_misc_2() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sbc(0x1F);

        assert_eq!(0xE1, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn sbc_misc_3() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sbc(1);

        assert_eq!(0xFF, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }

    #[test]
    fn sbc_misc_4() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sbc(0xFF);

        assert_eq!(1, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(true, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }
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

#[cfg(test)]
mod inc {
    use super::*;

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
}

#[cfg(test)]
mod dec {
    use super::*;

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
}

#[test]
fn daa() {
    let mut cpu = setup();

    cpu.set_reg(Reg::AF, 0x9A00);
    cpu.daa();

    assert_eq!(0x90, cpu.reg(Reg::AF));

    // Flags affected
    assert_eq!(true, cpu.flag(Flag::Zero));
    assert_eq!(false, cpu.flag(Flag::AddSub));
    assert_eq!(false, cpu.flag(Flag::HalfCarry));
    assert_eq!(true, cpu.flag(Flag::Carry));
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
fn di() {
    let mut cpu = setup();
    let mut bus = MockBus::default();

    cpu.ei();
    cpu.di(&mut bus);
    assert_eq!(false, bus.interrupts_enabled);

    // Flags affected
    assert_eq!(false, cpu.flag(Flag::Carry));
    assert_eq!(false, cpu.flag(Flag::Zero));
    assert_eq!(false, cpu.flag(Flag::AddSub));
    assert_eq!(false, cpu.flag(Flag::HalfCarry));
}

#[cfg(test)]
mod rlc {
    use super::*;

    #[test]
    fn rlc() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.rlc(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b10, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rlc_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.rlc(&mut bus, Reference::Register(Reg::A));
        assert_eq!(1, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rlc_zero() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.rlc(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }
}

#[cfg(test)]
mod rl {
    use super::*;

    #[test]
    fn rl_carry_off() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.rl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b10, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rl_carry_on() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.rl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b11, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rl_carry_on_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0001);
        cpu.rl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b11, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rl_zero() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.rl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }
}

#[cfg(test)]
mod rrc {
    use super::*;

    #[test]
    fn rrc() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.rrc(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b100_0000, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rrc_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.rrc(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b1000_0000, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rrc_zero() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.rrc(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }
}

#[cfg(test)]
mod rr {
    use super::*;

    #[test]
    fn rr_carry_off() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b10);
        cpu.rr(&mut bus, Reference::Register(Reg::A));
        assert_eq!(1, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rr_carry_on() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b10);
        cpu.rr(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b1000_0001, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rr_carry_on_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.set_flag(Flag::Carry, true);
        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0001);
        cpu.rr(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b1100_0000, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn rr_zero() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.rr(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }
}

#[cfg(test)]
mod swap {
    use super::*;

    #[test]
    fn swap() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0xEF);
        cpu.swap(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0xFE, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn swap_zero() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.swap(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod sla {
    use super::*;

    #[test]
    fn shifts_to_carry() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.sla(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        cpu.ld(&mut bus, Reference::Register(Reg::A), 1);
        cpu.sla(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b10, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn sets_zero_flag() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.sla(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }

    #[test]
    fn sla_misc() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::AF), 0);
        cpu.ld(&mut bus, Reference::Register(Reg::B), 0x80);
        cpu.sla(&mut bus, Reference::Register(Reg::B));
        assert_eq!(0, cpu.reg(Reg::B));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(true, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod sra {
    use super::*;

    #[test]
    fn shifts_to_carry_on() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b11);
        cpu.sra(&mut bus, Reference::Register(Reg::A));
        assert_eq!(1, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn shifts_to_carry_off() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b10);
        cpu.sra(&mut bus, Reference::Register(Reg::A));
        assert_eq!(1, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn msb_does_not_change() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.sra(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b1100_0000, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn sets_zero_flag() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.sra(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod srl {
    use super::*;

    #[test]
    fn shifts_to_carry_on() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b11);
        cpu.srl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(1, cpu.reg(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn shifts_to_carry_off() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b10);
        cpu.srl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(1, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn msb_set_to_zero() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.srl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0b0100_0000, cpu.reg(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Carry));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn sets_zero_flag() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.srl(&mut bus, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod bit {
    use super::*;

    #[test]
    fn tests_bits_correctly() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1010_1010);

        cpu.bit(&mut bus, 0, Reference::Register(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 1, Reference::Register(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 2, Reference::Register(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 3, Reference::Register(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 4, Reference::Register(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 5, Reference::Register(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 6, Reference::Register(Reg::A));
        assert_eq!(true, cpu.flag(Flag::Zero));

        cpu.bit(&mut bus, 7, Reference::Register(Reg::A));
        assert_eq!(false, cpu.flag(Flag::Zero));
    }

    #[test]
    fn sets_correct_flags() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.bit(&mut bus, 0, Reference::Register(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(true, cpu.flag(Flag::HalfCarry));
    }

    #[test]
    fn sets_zero_flag() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.bit(&mut bus, 0, Reference::Register(Reg::A));
        assert_eq!(0, cpu.reg(Reg::A));

        // Flags affected
        assert_eq!(true, cpu.flag(Flag::Zero));
    }
}

#[cfg(test)]
mod set {
    use super::*;

    #[test]
    fn sets_bits_correctly() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b1000_0000);
        cpu.set(&mut bus, 0, Reference::Register(Reg::A));
        cpu.set(&mut bus, 1, Reference::Register(Reg::A));
        cpu.set(&mut bus, 3, Reference::Register(Reg::A));

        assert_eq!(0b1000_1011, cpu.reg(Reg::A));
    }

    #[test]
    fn does_not_set_any_flags() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.set(&mut bus, 0, Reference::Register(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }
}

#[cfg(test)]
mod res {
    use super::*;

    #[test]
    fn resets_bits_correctly() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0b0110_1111);
        cpu.res(&mut bus, 0, Reference::Register(Reg::A));
        cpu.res(&mut bus, 1, Reference::Register(Reg::A));
        cpu.res(&mut bus, 3, Reference::Register(Reg::A));

        assert_eq!(0b0110_0100, cpu.reg(Reg::A));
    }

    #[test]
    fn does_not_set_any_flags() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::A), 0);
        cpu.res(&mut bus, 0, Reference::Register(Reg::A));

        // Flags affected
        assert_eq!(false, cpu.flag(Flag::Zero));
        assert_eq!(false, cpu.flag(Flag::AddSub));
        assert_eq!(false, cpu.flag(Flag::HalfCarry));
        assert_eq!(false, cpu.flag(Flag::Carry));
    }
}

#[test]
fn jp() {
    let mut cpu = setup();
    let mut bus = MockBus::default();

    cpu.jp(0xFE);
    assert_eq!(0xFE, cpu.reg(Reg::PC));
}

#[cfg(test)]
mod jr {
    use super::*;

    #[test]
    fn jumps_to_correct_address() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::PC), 1);
        cpu.jr(-1);
        assert_eq!(0, cpu.reg(Reg::PC));
        cpu.jr(1);
        assert_eq!(1, cpu.reg(Reg::PC));
    }

    #[test]
    fn handles_overflow() {
        let mut cpu = setup();
        let mut bus = MockBus::default();

        cpu.ld(&mut bus, Reference::Register(Reg::PC), 0);
        cpu.jr(-1);
        assert_eq!(std::u16::MAX, cpu.reg(Reg::PC));
        cpu.jr(1);
        assert_eq!(0, cpu.reg(Reg::PC));
    }
}

#[test]
fn call() {
    let mut cpu = setup();
    let mut bus = MockBus::default();
    let address = 0xFE00;
    let next_instruction = 0xF000;

    cpu.set_reg(Reg::PC, next_instruction);

    cpu.call(&mut bus, address);
    assert_eq!(address, cpu.reg(Reg::PC));

    let lower = cpu.registers.stack_pointer.pop(&bus) as u16;
    let higher = cpu.registers.stack_pointer.pop(&bus) as u16;

    assert_eq!(next_instruction, lower | (higher << 8))
}

#[test]
fn call_and_ret() {
    let mut cpu = setup();
    let mut bus = MockBus::default();
    let stack_pointer = cpu.reg(Reg::SP);
    let base_address = 0xF000;
    let address = 0xFE00;

    cpu.set_reg(Reg::PC, base_address);

    cpu.call(&mut bus, address);
    assert_eq!(address, cpu.reg(Reg::PC));
    assert_eq!(stack_pointer - 2, cpu.reg(Reg::SP));

    cpu.ret(&mut bus);
    assert_eq!(stack_pointer, cpu.reg(Reg::SP));
    assert_eq!(base_address, cpu.reg(Reg::PC))
}
