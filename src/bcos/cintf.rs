use libc::{c_char, c_int, c_uint, c_ulong, c_void};
use std::{slice, sync::mpsc};

use crate::bcos::macros::to_string;

#[repr(C)]
#[derive(Debug)]
pub struct BcosSdkCStructResponse {
    error: c_int,
    desc: *const c_char,
    data: *const c_void,
    size: c_ulong,
    context: *const c_void,
}

pub type BcosSdkCallback = extern "C" fn(*const BcosSdkCStructResponse);

pub extern "C" fn session_bcos_sdk_callback(bcos_sdk_response: *const BcosSdkCStructResponse) {
    let resp = unsafe { &*bcos_sdk_response };

    let tx: Box<mpsc::Sender<Vec<c_char>>> =
        unsafe { Box::from_raw(resp.context as *mut mpsc::Sender<_>) };

    if resp.error != 0 {
        panic!("{}", &to_string(resp.desc));
    }

    tx.send(unsafe { slice::from_raw_parts(resp.data as *const c_char, resp.size as usize) }.to_vec()).unwrap();
}

#[link(name = "bcos-c-sdk")]
extern "C" {
    pub fn bcos_sdk_version() -> *const c_char;
    pub fn bcos_sdk_create_by_config_file(config_file: *const c_char) -> *mut c_void;
    pub fn bcos_sdk_start(sdk: *mut c_void);
    pub fn bcos_sdk_stop(sdk: *mut c_void);
    pub fn bcos_sdk_destroy(sdk: *mut c_void);
    pub fn bcos_sdk_is_last_opr_success() -> c_int;
    pub fn bcos_sdk_get_last_error() -> c_int;
    pub fn bcos_sdk_get_last_error_msg() -> *const c_char;

    pub fn bcos_rpc_call(
        sdk: *mut c_void,
        group: *const c_char,
        node: *const c_char,
        to: *const c_char,
        data: *const c_char,
        callback: BcosSdkCallback,
        context: *const c_void,
    );

    pub fn bcos_rpc_send_transaction(
        sdk: *mut c_void,
        group: *const c_char,
        node: *const c_char,
        data: *const c_char,
        proof: c_int,
        callback: BcosSdkCallback,
        context: *const c_void,
    );

    pub fn bcos_sdk_create_signed_transaction(
        key_pair: *const c_void,
        group_id: *const c_char,
        chain_id: *const c_char,
        to: *const c_char,
        data: *const c_char,
        abi: *const c_char,
        block_limit: i64,
        attribute: i32,
        tx_hash: *mut *const c_char,
        signed_tx: *mut *const c_char,
    );

    pub fn bcos_sdk_get_group_chain_id(sdk: *mut c_void, group_id: *const c_char) -> *mut c_char;

    pub fn bcos_rpc_get_block_limit(sdk: *mut c_void, group: *const c_char) -> i64;

    pub fn bcos_event_sub_subscribe_event(
        sdk: *mut c_void,
        group: *const c_char,
        params: *const c_char,
    ) -> *mut c_char;

    pub fn bcos_event_sub_unsubscribe_event(sdk: *mut c_void, event_sub_id: *const c_char);

    pub fn bcos_sdk_abi_encode_constructor(
        abi: *const c_char,
        bin: *const c_char,
        params: *const c_char,
        crypto_type: c_int,
    ) -> *mut c_char;

    pub fn bcos_sdk_abi_encode_method(
        abi: *const c_char,
        method_name: *const c_char,
        params: *const c_char,
        crypto_type: c_int,
    ) -> *mut c_char;

    pub fn bcos_sdk_abi_decode_method_output(
        abi: *const c_char,
        method_name: *const c_char,
        data: *const c_char,
        crypto_type: c_int,
    ) -> *mut c_char;

    pub fn bcos_sdk_abi_decode_event(
        abi: *const c_char,
        event_name: *const c_char,
        data: *const c_char,
        crypto_type: c_int,
    ) -> *mut c_char;

    pub fn bcos_sdk_create_keypair_by_private_key(
        crypto_type: c_int,
        private_key: *const c_void,
        length: c_uint,
    ) -> *mut c_void;

    pub fn bcos_sdk_create_keypair_by_hex_private_key(
        crypto_type: c_int,
        private_key: *const c_char,
    ) -> *mut c_void;

    pub fn bcos_sdk_create_keypair(
        crypto_type: c_int,
    ) -> *mut c_void;

    pub fn bcos_sdk_get_keypair_public_key(
        keypair: *const c_void,
    ) -> *mut c_char;

    pub fn bcos_sdk_get_keypair_private_key(
        keypair: *const c_void,
    ) -> *mut c_char;

    pub fn bcos_sdk_get_keypair_address(
        keypair: *const c_void,
    ) -> *mut c_char;

    pub fn bcos_sdk_destroy_keypair(keypair: *mut c_void);

    pub fn bcos_sdk_c_free(p: *mut c_void);
}
