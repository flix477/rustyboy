use glium::glutin::dpi::LogicalSize;
use glium::glutin::{
    ContextBuilder, ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder,
    WindowEvent,
};
use glium::Display;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy};
use rustyboy_core::hardware::joypad::{Button, Input, InputType};

use self::screen::MainWindow;

pub mod background;
pub mod screen;
pub mod tile_data;

pub trait Window {
    fn update(&self, gameboy: &Gameboy);
}

pub fn create_display(
    title: &str,
    events_loop: &EventsLoop,
    dimensions: (usize, usize),
) -> Display {
    let window = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(LogicalSize {
            width: dimensions.0 as f64,
            height: dimensions.1 as f64,
        });
    let ctx = ContextBuilder::new();
    Display::new(window, ctx, events_loop).unwrap()
}

fn run(config: Config) {
    let mut gameboy = Gameboy::new(config).unwrap();

    let mut events_loop = EventsLoop::new();

    let main_window = MainWindow::new(&events_loop);

    let mut closed = false;
    while !closed {
        gameboy.run_to_vblank();

        main_window.update(&gameboy);

        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                closed = true;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                let input = keymap(input);
                if let Some(input) = input {
                    gameboy.send_input(input);
                }
            }
            _ => {}
        });
    }
}

fn keymap(input: KeyboardInput) -> Option<Input> {
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

fn main() {
    //    let cartridge = Cartridge::from_file("test/cpu_instrs.gb").unwrap();
    let cartridge = Cartridge::from_file("tetris.gb").unwrap();
    println!("{:?}", cartridge.metadata());
    let config = Config {
        cartridge,
        device_type: DeviceType::GameBoy,
        debugger_config: None,
    };

    run(config);
}
