use libc::c_char;
use std::ffi::{CString, CStr};

pub fn to_string(p: *const c_char) -> String {
    String::from_utf8_lossy(unsafe {
        CStr::from_ptr(p)
    }.to_bytes()).to_string()
}

pub fn to_cstring(s: &str) -> *const c_char {
    CString::new(s).unwrap().as_ptr()
}
