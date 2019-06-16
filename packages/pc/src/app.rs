use clap::App;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::debugger::Debugger;
use rustyboy_core::gameboy::{DeviceType, Gameboy};
use std::process::exit;

use crate::shell_debugger::{DebuggerState, ShellDebugger};
use crate::window::{background::BackgroundWindow, screen::MainWindow, Window};
use rustyboy_core::cartridge::cartridge_metadata::CartridgeMetadata;

pub fn run() {
    let matches = App::new("rustyboy")
        .version(crate_version!())
        .about("Gameboy emulator written in Rust.")
        .args_from_usage(
            "<rom_path> 'ROM path'
            -d, --debug 'Enable debugger'
            -i, --info 'Print cartridge metadata'
            -b, --background 'Display background contents'",
        )
        .get_matches();

    let path = matches.value_of("rom_path").unwrap();
    let cartridge = Cartridge::from_file(path).unwrap();

    if matches.is_present("info") {
        print_cartridge_info(cartridge.metadata());
        exit(0);
    }

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
    start_emulation(cartridge, config, matches.is_present("background"));
}

fn print_cartridge_info(metadata: &CartridgeMetadata) {
    println!("Cartridge metadata");
    println!("{}", "-".repeat(20));

    println!("Title: {}", metadata.title);
    println!("Region: {:?}", metadata.destination);
    println!("Version: {}", metadata.version);
    println!("GameBoy Color support: {:?}", metadata.cgb_flag);
    println!("Super GameBoy enhancements: {:?}", metadata.sgb_enhanced);
    println!("Manufacturer code: {:?}", metadata.manufacturer_code);
    println!("New licencee code: {:?}", metadata.new_licensee_code);
    println!("Old licencee code: {:?}", metadata.old_licensee_code);
    println!("Cartridge capabilities: {:?}", metadata.capabilities);
    println!("ROM size: {:?}", metadata.rom_size);
    println!("RAM size: {:?}", metadata.ram_size);
}

fn start_emulation(cartridge: Cartridge, config: Config, show_background: bool) {
    let mut gameboy = Gameboy::new(cartridge, config).unwrap();

    let mut windows = create_windows(show_background);

    loop {
        gameboy.run_to_vblank();

        for window in &mut windows {
            window.update(&mut gameboy);
        }
    }
}

fn create_windows(show_background: bool) -> Vec<Box<dyn Window>> {
    let main_window = MainWindow::new();
    let mut windows: Vec<Box<Window>> = vec![Box::new(main_window)];
    if show_background {
        windows.push(Box::new(BackgroundWindow::new()));
    }
    windows
}
