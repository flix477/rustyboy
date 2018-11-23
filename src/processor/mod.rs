mod processor;
mod register;
mod flag_register;
mod instruction;
mod decoder;
mod lr35902;
mod registers;
mod program_counter;
mod stack_pointer;
//
//use self::decoder::Decoder;
//use self::processor::Processor;
//use self::super::memory::Memory;
//use self::lr35902::LR35902;
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn implemented_coverage() {
//        let mut cpu = Processor::new(Memory::new());
//        let mut covered = Vec::new();
//        for i in 0..0xff {
//            if let Some(instruction) = Decoder::decode_opcode(i) {
//                if let Ok(_) = cpu.execute(instruction) {
//                    covered.push(i);
//                }
//            }
//        }
//        assert_eq!(covered.len(), 0xff - 11);
//    }
//}