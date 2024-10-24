use libc::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct BcosSdkCStructResponse {
    pub error: c_int,
    pub desc: *const c_char,
    pub data: *const c_void,
    pub size: c_ulong,
    pub context: *const c_void,
}

pub type BcosSdkCallback = extern "C" fn(resp: *const BcosSdkCStructResponse);

#[link(name = "bcos-c-sdk")]
extern "C" {
    pub fn bcos_sdk_version() -> *const c_char;
    pub fn bcos_sdk_create_by_config_file(config_file: *const c_char) -> *const c_void;
    pub fn bcos_sdk_start(sdk: *const c_void);
    pub fn bcos_sdk_stop(sdk: *const c_void);
    pub fn bcos_sdk_destroy(sdk: *const c_void);
    pub fn bcos_sdk_get_last_error_msg() -> *const c_char;
    pub fn bcos_sdk_is_last_opr_success() -> c_int;
    pub fn bcos_sdk_get_last_error() -> c_int;

    pub fn bcos_rpc_call(
        sdk: *const c_void,
        group: *const c_char,
        node: *const c_char,
        to: *const c_char,
        data: *const c_char,
        callback: BcosSdkCallback,
        context: *const c_void,
    );

    pub fn bcos_rpc_send_transaction(
        sdk: *const c_void,
        group: *const c_char,
        node: *const c_char,
        data: *const c_char,
        proof: c_int,
        callback: BcosSdkCallback,
        context: *const c_void,
    );
}
