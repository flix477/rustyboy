use rustyboy_core::hardware::joypad::{Button, Input, InputType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InputButton {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

impl Into<Button> for InputButton {
    fn into(self) -> Button {
        match self {
            InputButton::A => Button::A,
            InputButton::B => Button::B,
            InputButton::Start => Button::Start,
            InputButton::Select => Button::Select,
            InputButton::Up => Button::Up,
            InputButton::Down => Button::Down,
            InputButton::Left => Button::Left,
            InputButton::Right => Button::Right,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InputTypeJs {
    Down,
    Up,
}

impl Into<InputType> for InputTypeJs {
    fn into(self) -> InputType {
        match self {
            InputTypeJs::Down => InputType::Down,
            InputTypeJs::Up => InputType::Up,
        }
    }
}

#[wasm_bindgen(js_name = Input)]
pub struct InputJs {
    input_type: InputTypeJs,
    button: InputButton,
}

#[wasm_bindgen(js_class = Input)]
impl InputJs {
    #[wasm_bindgen(constructor)]
    pub fn new(input_type: InputTypeJs, button: InputButton) -> Self {
        InputJs { input_type, button }
    }
}

impl Into<Input> for InputJs {
    fn into(self) -> Input {
        Input {
            button: self.button.into(),
            input_type: self.input_type.into(),
        }
    }
}
