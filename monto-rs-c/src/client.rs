use std::ptr::null_mut;

use futures::future::Future;
use libc::{c_char, c_uint};
use monto::client::{Client, Config};
use monto::common::messages::SoftwareVersion;
use tokio_core::reactor::Core;
use util::{cstr, cstr_or};

#[repr(C)]
pub struct ClientConfig {
	hostname: *const c_char,
    port: u16,
	identifier: *const c_char,
	name: *const c_char,
	vendor: *const c_char,
    major: c_uint,
    minor: c_uint,
    patch: c_uint,
}

pub struct FFIClient {
    core: Core,
    client: Client,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn monto_rs_Client_new(config: ClientConfig) -> *mut FFIClient {
    let id = if let Some(id) = cstr(config.identifier) {
        if let Ok(id) = id.parse() {
            id
        } else {
            return null_mut();
        }
    } else {
        return null_mut();
    };
    let config = Config {
        host: cstr_or(config.hostname, "localhost"),
        port: if config.port == 0 { 28888 } else { config.port },
        version: SoftwareVersion {
            id: id,
            name: cstr(config.name),
            vendor: cstr(config.vendor),
            major: config.major as u64,
            minor: config.minor as u64,
            patch: config.patch as u64,
        },
    };

    let core = if let Ok(core) = Core::new() {
        core
    } else {
        return null_mut();
    };

    if let Ok(client) = Client::new(config, core.handle()).wait() {
        Box::into_raw(Box::new(FFIClient { client, core }))
    } else {
        null_mut()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn monto_rs_Client_free(client: *mut FFIClient) {
    let boxed = unsafe {
        Box::from_raw(client)
    };
    drop(boxed);
}
