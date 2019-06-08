use crate::bus::{Readable, Writable};
use crate::util::bitflags::Bitflags;

pub struct Joypad {
    mode: Mode,
    pushed_keys: u8,
}

impl Joypad {
    pub fn new() -> Joypad {
        Self::default()
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn send_input(&mut self, input: Input) {
        self.set_flag(input.button, input.input_type == InputType::Down)
    }
}

impl Default for Joypad {
    fn default() -> Joypad {
        Joypad {
            mode: Mode::DirectionalKeys,
            pushed_keys: 0,
        }
    }
}

impl Readable for Joypad {
    fn read(&self, _: u16) -> u8 {
        (self.mode as u8)
            | if let Mode::DirectionalKeys = self.mode {
                !self.pushed_keys >> 4
            } else {
                !self.pushed_keys & 0xF
            }
    }
}

impl Writable for Joypad {
    fn write(&mut self, _: u16, value: u8) {
        match value {
            0x20 => {
                self.mode = Mode::DirectionalKeys;
            }
            0x10 => {
                self.mode = Mode::ButtonKeys;
            }
            _ => {}
        };
    }
}

impl Bitflags<Button> for Joypad {
    fn register(&self) -> u8 {
        self.pushed_keys
    }

    fn set_register(&mut self, value: u8) {
        self.pushed_keys = value;
    }
}

pub struct Input {
    pub input_type: InputType,
    pub button: Button,
}

#[derive(PartialEq)]
pub enum InputType {
    Down,
    Up,
}

pub enum Button {
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Right = 16,
    Left = 32,
    Up = 64,
    Down = 128,
}

impl Into<u8> for Button {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone)]
pub enum Mode {
    DirectionalKeys = 0x20,
    ButtonKeys = 0x10,
}

impl Mode {
    pub fn from_value(value: u8) -> Mode {
        match value {
            0x20 => Mode::DirectionalKeys,
            0x10 => Mode::ButtonKeys,
            _ => panic!("Invalid value."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_joypad_info_none() {
        let joypad = Joypad::new();
        assert_eq!(joypad.read(0), 0x2F);
    }

    #[test]
    fn fetch_joypad_info_some_wrong_mode() {
        let mut joypad = Joypad::new();
        joypad.set_flag(Button::A, true);
        assert_eq!(joypad.read(0), 0x2F);
    }

    #[test]
    fn fetch_joypad_info_some() {
        let mut joypad = Joypad::new();
        joypad.set_mode(Mode::ButtonKeys);
        joypad.set_flag(Button::A, true);
        assert_eq!(joypad.read(0), 0x1E);
    }
}
