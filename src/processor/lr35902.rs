use processor::instruction::InstructionInfo;

pub trait LR35902 {
    fn ld(&mut self, instruction: InstructionInfo);
}