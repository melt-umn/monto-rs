/// A trait for representing HTTP status codes at the type level.
pub trait StatusCode {
    /// The value-level status code corresponding to this type-level status
    /// code.
    const VALUE: ::hyper::StatusCode;
}

include!(concat!(env!("OUT_DIR"), "/status_types.rs"));
