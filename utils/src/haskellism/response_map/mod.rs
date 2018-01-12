//! A map between HTTP status codes and response types, for encoding the
//! response space of an HTTP server.

mod deserialize_body;
pub mod status_types;

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use haskellism::nat::{NSucc, NZero, Nat};
pub use haskellism::response_map::deserialize_body::deserialize_body;
use haskellism::response_map::status_types::StatusCode;

/// A trait for response bodies.
pub trait RespBody<S: StatusCode>: for<'de> Deserialize<'de> + Serialize {}

/// An error when deserializing a response.
pub enum RespError {
    /// A response code was present that isn't handled.
    BadStatus(::hyper::StatusCode),

    /// The body wasn't able to be deserialized.
    Json(::serde_json::Error),
}

/// A trait for a type-level map between response codes and structures to
/// deserialize response bodies to.
pub trait RespMap<S: StatusCode, T: RespBody<S>, Idx: Nat> {
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body(s: &str) -> Result<T, RespError>;
}

/// A default handler at the end of a `RespMap`, which always returns a
/// `BadStatus` error.
pub struct RespMapNil;

impl<S: StatusCode, T: RespBody<S>> RespMap<S, T, NZero> for RespMapNil {
    fn deserialize_body(_: &str) -> Result<T, RespError> {
        Err(RespError::BadStatus(S::VALUE))
    }
}

/// A status-handler pair placed in front of another `RespMap`.
pub struct RespMapCons<S, T, Tl>(PhantomData<(S, T, Tl)>);

impl<S, T, Tl> RespMap<S, T, NZero> for RespMapCons<S, T, Tl>
where
    S: StatusCode,
    T: RespBody<S>,
{
    fn deserialize_body(s: &str) -> Result<T, RespError> {
        ::serde_json::from_str(s).map_err(RespError::Json)
    }
}

impl<S, T, Tl, TlS, TlT, N> RespMap<TlS, TlT, NSucc<N>>
    for RespMapCons<S, T, Tl>
where
    Tl: RespMap<TlS, TlT, N>,
    TlS: StatusCode,
    TlT: RespBody<TlS>,
    N: Nat,
{
    fn deserialize_body(s: &str) -> Result<TlT, RespError> {
        ::serde_json::from_str(s).map_err(RespError::Json)
    }
}
