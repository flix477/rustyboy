use crate::Gameboy;
use rustyboy_core::hardware::joypad::{Button, Input, InputType as RustInputType};

#[repr(C)]
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InputType {
    InputDown,
    InputUp,
}

impl Into<RustInputType> for InputType {
    fn into(self) -> RustInputType {
        match self {
            InputType::InputDown => RustInputType::Down,
            InputType::InputUp => RustInputType::Up,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_send_input(
    gameboy: *mut Gameboy,
    button: InputButton,
    input_type: InputType,
) {
    let mut gameboy = {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };
    gameboy.gameboy.send_input(Input {
        button: button.into(),
        input_type: input_type.into(),
    });
    Box::into_raw(gameboy);
}
