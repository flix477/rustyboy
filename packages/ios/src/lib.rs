use std::os::raw::{c_uchar, c_ulong};
use std::slice;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{DeviceType, Gameboy as RustGameboy};
use rustyboy_core::video::color::ColorFormat;

pub mod input;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_BUFFER_SIZE: usize = 69_120;

pub struct Gameboy {
    gameboy: RustGameboy,
}

#[no_mangle]
pub unsafe extern "C" fn create_gameboy(buffer: *const c_uchar, length: c_ulong) -> *mut Gameboy {
    assert!(!buffer.is_null(), "Cartridge buffer is null");
    let buffer: &[c_uchar] = slice::from_raw_parts(buffer, length as usize);

    let cartridge = Cartridge::from_buffer(buffer.to_vec()).unwrap();
    let config = Config {
        debugger: None,
        device_type: DeviceType::GameBoy,
    };

    let gameboy = RustGameboy::new(cartridge, &config);
    let ffi_gameboy = Gameboy { gameboy };

    Box::into_raw(Box::new(ffi_gameboy))
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_run_to_vblank(gameboy: *mut Gameboy) -> *mut c_uchar {
    let mut gameboy = {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };
    gameboy.gameboy.run_to_vblank();
    let mut buffer: Box<[u8]> = gameboy
        .gameboy
        .hardware()
        .video()
        .screen()
        .buffer(ColorFormat::RGBA)
        .into_boxed_slice();

    let pointer: *mut c_uchar = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    Box::into_raw(gameboy);

    pointer
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_free(gameboy: *mut Gameboy) {
    if gameboy.is_null() {
        return;
    }
    Box::from_raw(gameboy);
}

#[no_mangle]
pub unsafe extern "C" fn buffer_free(buffer: *mut c_uchar) {
    if buffer.is_null() {
        return;
    }
    Box::from_raw(slice::from_raw_parts_mut(buffer, SCREEN_BUFFER_SIZE));
}
