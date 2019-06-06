use clap::App;
use glium::glutin::{Event, EventsLoop, WindowEvent};

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::debugger::Debugger;
use rustyboy_core::gameboy::{DeviceType, Gameboy};

use crate::keymap::keymap;
use crate::shell_debugger::{DebuggerState, ShellDebugger};
use crate::window::{screen::MainWindow, Window};

pub fn run() {
    let matches = App::new("rustyboy")
        .version(crate_version!())
        .about("Gameboy emulator written in Rust.")
        .args_from_usage(
            "<rom_path> 'ROM path'
            -d, --debug 'Enable debugger'",
        )
        .get_matches();

    let path = matches.value_of("rom_path").unwrap();
    let cartridge = Cartridge::from_file(path).unwrap();

    let debugger = if matches.is_present("debug") {
        Some(Box::new(ShellDebugger::from_state(DebuggerState {
            forced_break: true,
            breakpoints: vec![],
        })) as Box<dyn Debugger>)
    } else {
        None
    };

    let config = Config {
        device_type: DeviceType::GameBoy,
        debugger,
    };
    start_emulation(cartridge, config);
}

fn start_emulation(cartridge: Cartridge, config: Config) {
    let mut gameboy = Gameboy::new(cartridge, config).unwrap();

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
