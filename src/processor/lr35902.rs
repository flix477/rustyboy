use processor::instruction::Operand;
use processor::registers::RegisterType;
use processor::instruction::{InstructionInfo, InstructionMnemonic};

pub trait LR35902 {
    fn execute(&mut self, instruction: InstructionInfo) {
        match *instruction.mnemonic() {
            InstructionMnemonic::LD => {
                if let Some(operands) = instruction.operands() {
                    self.ld(operands[0], operands[1]);
                } else {
                    panic!("LD needs two arguments");
                }
            },
            InstructionMnemonic::LDD => {
                if let Some(operands) = instruction.operands() {
                    self.ldd(operands[0], operands[1]);
                } else {
                    panic!("LDD needs two arguments");
                }
            },
            InstructionMnemonic::LDI => {
                if let Some(operands) = instruction.operands() {
                    self.ldi(operands[0], operands[1]);
                } else {
                    panic!("LDI needs two arguments");
                }
            },
            InstructionMnemonic::LDHL => self.ldhl(),
            InstructionMnemonic::PUSH => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Register(register) = operands[0] {
                        self.push(register);
                        return;
                    }
                }
                panic!("PUSH needs one register argument");
            },
            InstructionMnemonic::POP => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Register(register) = operands[0] {
                        self.pop(register);
                        return;
                    }
                }
                panic!("POP needs one register argument");
            },
            InstructionMnemonic::ADD => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Register(r1) = operands[0] {
                        self.add(r1, operands[1]);
                    }
                } else {
                    panic!("ADD needs two arguments");
                }
            }
            InstructionMnemonic::ADC => {
                if let Some(operands) = instruction.operands() {
                    if let Operand::Register(r1) = operands[0] {
                        self.adc(r1, operands[1]);
                    }
                } else {
                    panic!("ADC needs two arguments");
                }
            }
            _ => {}
        }
    }

    fn ld(&mut self, op1: Operand, op2: Operand);
    fn ldd(&mut self, op1: Operand, op2: Operand);
    fn ldi(&mut self, op1: Operand, op2: Operand);
    fn ldhl(&mut self);
    fn push(&mut self, register: RegisterType);
    fn pop(&mut self, register: RegisterType);
    fn add(&mut self, register: RegisterType, op: Operand);
    fn adc(&mut self, register: RegisterType, op: Operand);
    fn sub(&mut self, op: Operand);
}