use glium::glutin::{ElementState, KeyboardInput, VirtualKeyCode};

use rustyboy_core::hardware::joypad::{Button, Input, InputType};

pub fn keymap(input: KeyboardInput) -> Option<Input> {
    let key_code = input.virtual_keycode?;
    let button = match key_code {
        VirtualKeyCode::Up => Button::Up,
        VirtualKeyCode::Down => Button::Down,
        VirtualKeyCode::Left => Button::Left,
        VirtualKeyCode::Right => Button::Right,
        VirtualKeyCode::Return => Button::Start,
        VirtualKeyCode::Space => Button::Select,
        VirtualKeyCode::X => Button::B, // TODO: use scancode for those so keymaps dont change the position
        VirtualKeyCode::Z => Button::A,
        _ => return None,
    };

    let input_type = if input.state == ElementState::Pressed {
        InputType::Down
    } else {
        InputType::Up
    };

    Some(Input { input_type, button })
}
