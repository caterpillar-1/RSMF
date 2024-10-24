use libc::c_void;
use crate::bcos::cintf::*;
use std::ffi::{CString, CStr};
use super::macros::*;

pub struct Sdk {
    sdk: *const c_void,
}

impl Sdk {
    pub fn new(config: &str) -> Self {
        Sdk {
            sdk: unsafe {
                bcos_sdk_create_by_config_file(to_cstring(config))
            }
        }
    }

    pub fn start(&mut self) {
        unsafe {
            bcos_sdk_start(self.sdk);
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            bcos_sdk_stop(self.sdk);
        }
    }

    pub fn version(&self) -> String {
        to_string(unsafe {bcos_sdk_version() })
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        unsafe {
            bcos_sdk_destroy(self.sdk);
        }
    }
}
