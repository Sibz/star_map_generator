extern crate libc;
extern crate star_map_generator;

mod ffi_result;

use ffi_result::result;
use libc::c_char;
use star_map_generator::*;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn generate_star_map_in_preallocated_memory(
    options: StarMapOptions,
    pointer: *mut StarMapEntry,
) -> result {
    let r = std::panic::catch_unwind(|| {
        let generate_result =
            star_map_generator::generate_star_map_in_preallocated_memory(options, pointer);
        return match generate_result {
            Result::Ok(_) => result::new(0, ""),
            Err(err_reason) => result::new(1, &err_reason),
        };
    });
    match r {
        Result::Ok(rus) => rus,
        Err(_) => result::new(1, "Generate Star Map Panicked!"),
    }
}

#[no_mangle]
pub extern "C" fn generate_star_map_in_preallocated_memory_with_chunk_size(
    options: StarMapOptions,
    pointer: *mut StarMapEntry,
    chunk_size: u32
) -> result {
    let r = std::panic::catch_unwind(|| {
        let generate_result =
            star_map_generator::generate_star_map_in_preallocated_memory_with_chunk_size(options, pointer, chunk_size as usize);
        return match generate_result {
            Result::Ok(_) => result::new(0, ""),
            Err(err_reason) => result::new(1, &err_reason),
        };
    });
    match r {
        Result::Ok(rus) => rus,
        Err(_) => result::new(1, "Generate Star Map Panicked!"),
    }
}

#[no_mangle]
pub extern "C" fn get_default_star_map_options() -> StarMapOptions {
    StarMapOptions::defaults()
}

#[no_mangle]
pub extern "C" fn result_message_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
