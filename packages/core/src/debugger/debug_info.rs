use super::debug_operand_parser::{DebugOperandParser, ReadableVec};
use crate::processor::decoder::Decoder;
use crate::processor::instruction::{
    Condition, InstructionInfo, Mnemonic, Operand, Prefix, Reference, ValueType, AddressType
};
use crate::processor::operand_parser::OperandParser;
use crate::processor::registers::register::Register;
use crate::processor::registers::Registers;

pub struct ProcessorDebugInfo {
    pub registers: Registers,
}

pub struct DebugInfo {
    pub cpu_debug_info: ProcessorDebugInfo,
    pub bus: Vec<u8>,
}

impl DebugInfo {
    pub fn current_line(&self) -> u16 {
        self.cpu_debug_info.registers.program_counter.get()
    }

    // TODO: refactor this? not very clean
    pub fn parse_all(&self, address: u16) -> Vec<DebugInstructionInfo> {
        let mut parser = DebugOperandParser::new(address, &self);
        let mut instructions = vec![];
        let mut overflow = false;

        loop {
            if let Some(instruction) = self.parse_instruction_with_parser(&mut parser) {
                instructions.push(instruction);
            }

            let pc = parser.program_counter().get();
            if !overflow && pc <= address {
                overflow = true;
            }

            if overflow && pc >= address {
                break;
            }
        }

        instructions.sort_by_key(|instruction| instruction.line);
        instructions
    }

    pub fn parse_instruction(&self, address: u16) -> Option<DebugInstructionInfo> {
        let mut parser = DebugOperandParser::new(address, &self);
        self.parse_instruction_with_parser(&mut parser)
    }

    pub fn parse_instruction_with_parser(
        &self,
        parser: &mut DebugOperandParser<'_>,
    ) -> Option<DebugInstructionInfo> {
        let bus = ReadableVec { value: &self.bus };
        let address = parser.program_counter().get();

        let instruction =
            Decoder::decode_opcode(parser.mut_program_counter().fetch(&bus), Prefix::None)?;
        let instruction = if let Mnemonic::CB = instruction.mnemonic() {
            Decoder::decode_opcode(parser.mut_program_counter().fetch(&bus), Prefix::CB)?
        } else {
            instruction
        };

        let parsed_operands = if let Some(operands) = instruction.operands() {
            operands
                .iter()
                .map(|operand| Self::parse_operand(&bus, parser, *operand))
                .collect()
        } else {
            vec![]
        };

        Some(DebugInstructionInfo {
            line: address,
            instruction,
            parsed_operands,
        })
    }

    fn parse_operand(
        bus: &ReadableVec<'_>,
        parser: &mut DebugOperandParser<'_>,
        operand: Operand,
    ) -> ParsedOperand {
        match operand {
            Operand::Value(value) => {
                let parsed_value = parser.operand_value(bus, value);
                ParsedOperand::Value((value, parsed_value))
            }
            Operand::Condition(condition) => {
                let parsed_condition = parser.operand_condition(condition);
                ParsedOperand::Condition((condition, parsed_condition))
            }
            Operand::Reference(reference) => {
                let parsed_reference = parser.reference(bus, reference);
                ParsedOperand::Reference((reference, parsed_reference))
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ParsedOperand {
    Value((ValueType, u16)),
    Condition((Condition, bool)),
    Reference((Reference, u16)),
}

impl ParsedOperand {
    /// If this represents an immediate value (whether 8bit or 16bit), returns the value else None
    pub fn immediate_value(self) -> Option<u16> {
        match self {
            ParsedOperand::Reference((Reference::Address(AddressType::Immediate), value))
            | ParsedOperand::Reference((Reference::Address(AddressType::IncImmediate), value))
            | ParsedOperand::Value((ValueType::Immediate, value))
            | ParsedOperand::Value((ValueType::Immediate16, value))
            | ParsedOperand::Value((ValueType::SignedImmediate, value))
            | ParsedOperand::Value((ValueType::Address(AddressType::Immediate), value))
            | ParsedOperand::Value((ValueType::Address(AddressType::IncImmediate), value)) => Some(value),
            _ => None
        }
    }
}

impl ToString for ParsedOperand {
    fn to_string(&self) -> String {
        match *self {
            ParsedOperand::Value((value_type, _)) => value_type.to_string(),
            ParsedOperand::Condition((condition, _)) => condition.to_string(),
            ParsedOperand::Reference((reference, _)) => reference.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DebugInstructionInfo {
    pub line: u16,
    pub instruction: InstructionInfo,
    pub parsed_operands: Vec<ParsedOperand>,
}

#[cfg(test)]
mod tests {
    use crate::processor::instruction::Mnemonic;
    use crate::processor::registers::Registers;
    use crate::tests::util::mock_debug_info;

    #[test]
    fn parse_all() {
        let debug_info = mock_debug_info(Registers::default(), vec![0; 0x10000]);

        let instructions: Vec<Mnemonic> = debug_info
            .parse_all(0)
            .iter()
            .map(|x| *x.instruction.mnemonic())
            .collect();

        assert_eq!(instructions, vec![Mnemonic::NOP; 0x10000])
    }
}
