use libc::c_char;
use std::ffi::{CString, CStr};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EMPTY_CSTRING: CString = CString::new("").unwrap();
}

pub fn to_string(p: *const c_char) -> String {
    String::from_utf8_lossy(unsafe {
        CStr::from_ptr(p)
    }.to_bytes()).to_string()
}
