use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ptr::null_mut;

use either::{Left, Right};
use futures::future::Future;
use libc::{c_char, c_uint};
use monto::common::messages::SoftwareVersion;
use monto::service::Service;
use monto::service::config::{Config, NetConfig};
use tokio_core::reactor::Core;
use void::unreachable;

use util::{cstr, cstr_parse, cstr_parse_or};

#[repr(C)]
pub struct ServiceConfig {
	addr: *const c_char,
	extensions: *const *const c_char,
    port: u16,

	identifier: *const c_char,
	name: *const c_char,
	vendor: *const c_char,
    major: c_uint,
    minor: c_uint,
    patch: c_uint,
}

pub struct FFIService {
    core: Core,
    service: Service,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn monto_rs_Service_new(config: ServiceConfig) -> *mut FFIService {
    let addr = cstr_parse_or(config.addr, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    let port = if config.port == 0 { 28888 } else { config.port };
    let addr = SocketAddr::new(addr, port);

    /*
    let id = if let Some(id) = cstr_parse(config.identifier) {
        id
    } else {
        return null_mut();
    };
    */
    let config = Config {
        extensions: unimplemented!(),
        net: NetConfig { addr },
        version: unimplemented!(),
        /*
        version: SoftwareVersion {
            id: id,
            name: cstr(config.name),
            vendor: cstr(config.vendor),
            major: config.major as u64,
            minor: config.minor as u64,
            patch: config.patch as u64,
        },
        */
    };

    let core = if let Ok(core) = Core::new() {
        core
    } else {
        return null_mut();
    };

    let service = Service::new(config, core.handle());
    Box::into_raw(Box::new(FFIService { service, core }))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn monto_rs_Service_free(service: *mut FFIService) {
    let boxed = unsafe {
        Box::from_raw(service)
    };
    drop(boxed);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn monto_rs_Service_serve_forever(service: *mut FFIService) {
    let boxed = unsafe {
        Box::from_raw(service)
    };

    let error = match boxed.service.serve_forever().wait() {
        Ok(void) => unreachable(void),
        Err(Left(void)) => unreachable(void),
        Err(Right(err)) => err,
    };

    println!("{}", error);
}
