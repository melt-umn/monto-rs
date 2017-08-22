use std::ffi::CStr;
use std::str::FromStr;

use libc::c_char;

pub fn cstr(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        let cstr = unsafe {
            CStr::from_ptr(ptr)
        };
        Some(cstr.to_string_lossy().to_string())
    }
}

pub fn cstr_or(ptr: *const c_char, default: &str) -> String {
    if let Some(s) = cstr(ptr) {
        s
    } else {
        default.to_string()
    }
}

pub fn cstr_parse<T: FromStr>(ptr: *const c_char) -> Option<T> {
    if let Some(s) = cstr(ptr) {
        s.parse().ok()
    } else {
        None
    }
}

pub fn cstr_parse_or<T: FromStr>(ptr: *const c_char, default: T) -> T {
    if let Some(s) = cstr_parse(ptr) {
        s
    } else {
        default
    }
}
