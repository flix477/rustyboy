use std::os::raw::{c_uchar, c_ulong};
use std::slice;

use rustyboy_core::cartridge::Cartridge;
use rustyboy_core::config::Config;
use rustyboy_core::gameboy::{Gameboy as RustGameboy, StepContext as RustStepContext};

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_BUFFER_SIZE: usize = 69_120;

pub struct Gameboy {
    gameboy: RustGameboy,
}

#[repr(C)]
pub struct Vector {
    size: c_ulong,
    pointer: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StepContext {
    pushed_keys: u8,
    serial_data_input: u8,
}

impl Into<RustStepContext> for StepContext {
    fn into(self) -> RustStepContext {
        RustStepContext {
            pushed_keys: self.pushed_keys,
            serial_data_input: None,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_gameboy(buffer: *const c_uchar, length: c_ulong) -> *mut Gameboy {
    assert!(!buffer.is_null(), "Cartridge buffer is null");
    let buffer: &[c_uchar] = slice::from_raw_parts(buffer, length as usize);

    let cartridge = Cartridge::from_buffer(buffer.to_vec()).unwrap();
    let config = Config::default();

    let gameboy = RustGameboy::new(cartridge, &config);
    let ffi_gameboy = Gameboy { gameboy };

    Box::into_raw(Box::new(ffi_gameboy))
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_run_to_vblank(
    gameboy: *mut Gameboy,
    context: StepContext,
) -> *mut c_uchar {
    let mut gameboy = {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };
    gameboy.gameboy.run_to_vblank(&context.into());
    let buffer = gameboy.gameboy.hardware().video.screen().buffer.rgba();
    let mut buffer = Box::new(buffer);

    let pointer: *mut c_uchar = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    Box::into_raw(gameboy);

    pointer
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_dump_savestate(
    gameboy: *mut Gameboy,
    out: *mut *mut c_uchar,
) -> c_ulong {
    let gameboy = {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };
    let mut savestate = gameboy.gameboy.dump_savestate();
    savestate.shrink_to_fit();
    let size = savestate.len() as c_ulong;

    *out = savestate.as_mut_ptr();

    std::mem::forget(savestate);
    Box::into_raw(gameboy);

    size
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_load_savestate(
    gameboy: *mut Gameboy,
    buffer: *const c_uchar,
    length: c_ulong,
) -> bool {
    let mut gameboy = {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };

    assert!(!buffer.is_null(), "Savestate buffer is null");
    let buffer: &[c_uchar] = slice::from_raw_parts(buffer, length as usize);

    let result = gameboy.gameboy.load_savestate(buffer.to_vec());
    Box::into_raw(gameboy);

    result.is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_reset(gameboy: *mut Gameboy) {
    let mut gameboy = {
        assert!(!gameboy.is_null(), "Gameboy is null");
        Box::from_raw(gameboy)
    };
    gameboy.gameboy.reset();
    Box::into_raw(gameboy);
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

#[no_mangle]
pub unsafe extern "C" fn vec_free(buffer: *mut c_uchar, length: c_ulong) {
    if buffer.is_null() {
        return;
    }
    Vec::from_raw_parts(buffer, length as usize, length as usize);
}
