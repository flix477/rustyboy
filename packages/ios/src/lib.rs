use std::os::raw::{c_uchar, c_ulong};

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy as RustGameboy};
use rustyboy_core::video::color::ColorFormat;
use rustyboy_core::video::screen::SCREEN_SIZE;
use std::slice;

pub struct Gameboy {
    gameboy: RustGameboy,
}

#[no_mangle]
pub extern "C" fn create_gameboy(buffer: *const c_uchar, length: c_ulong) -> *mut Gameboy {
    assert!(!buffer.is_null(), "Cartridge buffer is null");
    let buffer: &[c_uchar] = unsafe { slice::from_raw_parts(buffer, length as usize) };

    let cartridge = Cartridge::from_buffer(buffer.to_vec()).unwrap();
    let config = Config {
        debugger: None,
        device_type: DeviceType::GameBoy,
    };

    let gameboy = RustGameboy::new(cartridge, config);
    let ffi_gameboy = Gameboy { gameboy };

    Box::into_raw(Box::new(ffi_gameboy))
}

#[no_mangle]
pub extern "C" fn gameboy_run_to_vblank(gameboy: *mut Gameboy) -> *mut c_uchar {
    let mut gameboy = unsafe {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };
    gameboy.gameboy.run_to_vblank();
    let buffer = gameboy
        .gameboy
        .hardware()
        .video()
        .screen()
        .buffer(ColorFormat::RGB)
        .into_boxed_slice()
        .as_mut_ptr();
    Box::into_raw(gameboy);
    std::mem::forget(buffer);

    buffer
}

#[no_mangle]
pub extern "C" fn gameboy_free(gameboy: *mut Gameboy) {
    unsafe {
        if gameboy.is_null() {
            return;
        }
        Box::from_raw(gameboy);
    }
}

#[no_mangle]
pub extern "C" fn buffer_free(buffer: *mut c_uchar) {
    unsafe {
        if buffer.is_null() {
            return;
        }
        let s = slice::from_raw_parts_mut(buffer, SCREEN_SIZE.0 * SCREEN_SIZE.1);
        s.as_mut_ptr();
    }
}
