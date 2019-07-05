use crate::bus::Readable;
use crate::processor::instruction::{AddressType, Condition, ValueType};
use crate::processor::registers::flag_register::Flag;
use crate::processor::registers::program_counter::ProgramCounter;
use crate::processor::registers::RegisterType;

pub trait OperandParser {
    fn mut_program_counter(&mut self) -> &mut ProgramCounter;

    fn immediate<H: Readable>(&mut self, bus: &H) -> u8 {
        self.mut_program_counter().fetch(bus)
    }

    fn immediate16<H: Readable>(&mut self, bus: &H) -> u16 {
        u16::from(self.immediate(bus)) | (u16::from(self.immediate(bus)) << 8)
    }

    fn reg(&self, register: RegisterType) -> u16;
    fn flag(&self, flag: Flag) -> bool;

    fn operand_value<H: Readable>(&mut self, bus: &H, value: ValueType) -> u16 {
        match value {
            ValueType::Constant(value) => value,
            ValueType::Register(reg) => self.reg(reg),
            ValueType::Immediate | ValueType::SignedImmediate => u16::from(self.immediate(bus)),
            ValueType::Immediate16 => self.immediate16(bus),
            ValueType::Address(address) => {
                let address = self.operand_address(bus, address);
                u16::from(bus.read(address))
            }
        }
    }

    fn operand_address<H: Readable>(&mut self, bus: &H, address: AddressType) -> u16 {
        match address {
            AddressType::Register(reg) => self.reg(reg),
            AddressType::IncRegister(reg) => self.reg(reg).wrapping_add(0xFF00),
            AddressType::Immediate => self.immediate16(bus),
            AddressType::IncImmediate => u16::from(self.immediate(bus)).wrapping_add(0xFF00),
        }
    }

    fn operand_condition(&self, condition: Condition) -> bool {
        let Condition(flag, value) = condition;
        self.flag(flag) == value
    }
}
