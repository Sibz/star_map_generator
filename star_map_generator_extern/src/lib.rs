extern crate star_map_generator;

mod ffi_result;

use ffi_result::result;
use star_map_generator::*;

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
        Result::Ok(_) => result::new(500, "Generate did not return a result"),
        Err(_) => result::new(1, "Generate Star Map Panicked!"),
    }
}

pub extern "C" fn get_default_star_map_options() -> StarMapOptions {
    StarMapOptions::defaults()
}
