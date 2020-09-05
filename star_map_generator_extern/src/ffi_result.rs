extern crate libc;

use libc::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct result {
    exit_code: u16,
    failure_message: *mut c_char,
}

impl result {
    pub fn new(exit_code: u16, s: &str) -> result {
        result {
            exit_code,
            failure_message: CString::new(s).unwrap().into_raw(),
        }
    }
}
