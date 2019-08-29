use clap::App;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::debugger::Debugger;
use rustyboy_core::gameboy::{DeviceType, Gameboy, GameboyEvent};
use std::fs;
use std::process::exit;

use crate::shell_debugger::ShellDebugger;
use crate::window::background::BackgroundWindow;
use crate::window::tile_data::TileDataWindow;
use crate::window::{screen::MainWindow, UpdateResult, Window};
use rustyboy_core::cartridge::cartridge_metadata::CartridgeMetadata;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run() {
    let matches = App::new("rustyboy")
        .version(crate_version!())
        .about("Gameboy emulator written in Rust.")
        .args_from_usage(
            "<rom_path> 'ROM path'
            -d, --debug 'Enable debugger'
            -i, --info 'Print cartridge metadata'
            -b, --background 'Display background contents'
            -t --tiles 'Display tile data'
            -s --sprites 'Display sprite data'",
        )
        .get_matches();

    let path = matches.value_of("rom_path").unwrap();
    let cartridge = Cartridge::from_file(path).unwrap();

    if matches.is_present("info") {
        print_cartridge_info(cartridge.metadata());
        exit(0);
    }

    let debugger = if matches.is_present("debug") {
        Some(Debugger {
            forced_break: true,
            breakpoints: vec![],
        })
    } else {
        None
    };

    let config = Config {
        device_type: DeviceType::GameBoy,
        debugger,
    };

    let options = RunOptions {
        show_background: matches.is_present("background"),
        show_tile_data: matches.is_present("tiles"),
        show_sprite_data: matches.is_present("sprites"),
        path: PathBuf::from(path),
    };

    start_emulation(cartridge, config, options);
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

struct RunOptions {
    pub show_background: bool,
    pub show_tile_data: bool,
    pub show_sprite_data: bool,
    pub path: PathBuf,
}

fn start_emulation(cartridge: Cartridge, config: Config, options: RunOptions) {
    let mut gameboy = Gameboy::new(cartridge, &config);

    let mut windows = create_windows(&options);
    let mut debugger = config.debugger;
    let mut shell_debugger = ShellDebugger::default();

    let mut last_time = Instant::now();
    let update_rate = Duration::from_millis(1000 / 60);
    loop {
        let elapsed = last_time.elapsed();
        if elapsed < update_rate {
            continue;
        }
        last_time = Instant::now();

        if let GameboyEvent::Debugger(debug_info) = gameboy.run_to_event(debugger.as_mut()) {
            shell_debugger.run(debugger.as_mut().unwrap(), debug_info.as_ref())
        }

        if let UpdateResult::Close = update_windows(&mut gameboy, &mut windows) {
            if let Some(ram) = &gameboy.hardware().cartridge.ram {
                fs::write(options.path.with_extension("sav"), ram)
                    .expect("Could not save cartridge RAM; game progress might have been lost");
            }

            let savestate = &gameboy.dump_savestate();

            break;
        }
    }
}

fn update_windows(gameboy: &mut Gameboy, windows: &mut Vec<Box<dyn Window>>) -> UpdateResult {
    for window in windows.iter_mut() {
        if let UpdateResult::Close = window.update(gameboy) {
            return UpdateResult::Close;
        }
    }

    UpdateResult::Continue
}

fn create_windows(options: &RunOptions) -> Vec<Box<dyn Window>> {
    let main_window = MainWindow::new();
    let mut windows: Vec<Box<dyn Window>> = vec![Box::new(main_window)];

    if options.show_background {
        windows.push(Box::new(BackgroundWindow::new()))
    }

    if options.show_tile_data {
        windows.push(Box::new(TileDataWindow::new()))
    }

    windows
}
