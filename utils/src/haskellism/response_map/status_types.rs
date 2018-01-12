//! Type-level HTTP statuses.
//!
//! To add more, modify the `CODES` array in `build.rs`.

/// A trait for representing an HTTP status code at the type level.
pub trait StatusCode {
    /// The value-level status code corresponding to this type-level status
    /// code.
    const VALUE: ::hyper::StatusCode;
}

include!(concat!(env!("OUT_DIR"), "/status_types.rs"));
