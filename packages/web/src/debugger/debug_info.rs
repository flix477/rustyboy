use rustyboy_core::debugger::debug_info::DebugInfo;
use rustyboy_core::processor::instruction::Mnemonic;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = DebugInfo)]
pub struct DebugInfoJs {
    #[wasm_bindgen(skip)]
    pub debug_info: DebugInfo,
}

#[wasm_bindgen(js_class = DebugInfo)]
impl DebugInfoJs {
    pub fn bus(&self) -> Vec<u8> {
        // TODO: don't clone
        self.debug_info.bus.clone()
    }

    #[wasm_bindgen(js_name = parseAll)]
    pub fn parse_all(&self) -> JsValue {
        let pc = self.debug_info.current_line();
        let instructions: Vec<DebugInstructionInfoJs> = self
            .debug_info
            .parse_all(pc)
            .iter()
            .map(|x| DebugInstructionInfoJs {
                line: x.line,
                mnemonic: *x.instruction.mnemonic(),
                operands: x
                    .parsed_operands
                    .iter()
                    .map(|operand| {
                        if let Some(value) = operand.immediate_value() {
                            format!("{:X}", value)
                        } else {
                            operand.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(","),
            })
            .collect();
        JsValue::from_serde(&instructions).unwrap()
    }

    #[wasm_bindgen(js_name = currentLine)]
    pub fn current_line(&self) -> u16 {
        self.debug_info.current_line()
    }
}

#[derive(Serialize)]
pub struct DebugInstructionInfoJs {
    pub line: u16,
    #[serde(with = "MnemonicDef")]
    mnemonic: Mnemonic,
    operands: String,
}

#[derive(Serialize)]
#[serde(remote = "Mnemonic")]
pub enum MnemonicDef {
    CB,
    LD,
    LDHL,
    LDI,
    LDD,
    PUSH,
    POP,
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    INC,
    DEC,
    DAA,
    CPL,
    RLC,
    RLCA,
    RL,
    RLA,
    RRC,
    RRCA,
    RR,
    RRA,
    SLA,
    SWAP,
    SRA,
    SRL,
    BIT,
    SET,
    RES,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    JP,
    JR,
    CALL,
    RET,
    RETI,
    RST,
}
