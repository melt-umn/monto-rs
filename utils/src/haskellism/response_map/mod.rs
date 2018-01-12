//! Thanks to Francis Gagné on Stack Overflow for help with creating this
//! module: https://stackoverflow.com/questions/48220203#48221091

pub mod status_types;

use std::marker::PhantomData;

use serde::Deserialize;

use self::status_types::StatusCode;

/// An error when deserializing a response.
pub enum RespError {
    /// A response code was present that isn't handled.
    BadStatus(::hyper::StatusCode),

    /// The body wasn't able to be deserialized.
    Json(::serde_json::Error),
}

/// A trait for a type-level map between response codes and structures to
/// deserialize response bodies to.
///
/// If this looks incredibly strange and unfamiliar, look at [the Haskell HList
/// package](https://hackage.haskell.org/package/HList) or [Strongly Typed
/// Heterogeneous Collections](http://okmij.org/ftp/Haskell/HList-ext.pdf).
pub trait RespMap {
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body<S: StatusCode, T: for<'de> Deserialize<'de>>(
        s: &str,
    ) -> Result<T, RespError>;
}

/// A helper trait for `RespMap`.
pub trait RespMapImpl<S, T> {
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body(s: &str) -> Result<T, RespError>;
}

/// A default handler at the end of a `RespMap`, which always returns a
/// `BadStatus` error.
pub struct RespMapNil;

impl RespMap for RespMapNil {
    fn deserialize_body<S: StatusCode, T: for<'de> Deserialize<'de>>(
        _: &str,
    ) -> Result<T, RespError> {
        Err(RespError::BadStatus(S::VALUE))
    }
}

/// A status-handler pair placed in front of another `RespMap`.
pub struct RespMapCons<S, T, Tl>(PhantomData<(S, T, Tl)>);

impl<S, T, Tl> RespMap for RespMapCons<S, T, Tl>
where
    S: StatusCode,
    Tl: RespMap,
{
    fn deserialize_body<LS: StatusCode, LT: for<'de> Deserialize<'de>>(
        s: &str,
    ) -> Result<LT, RespError> {
        <Self as RespMapImpl<LS, LT>>::deserialize_body(s)
    }
}

default impl<S, T, Tl, LS, LT: for<'de> Deserialize<'de>> RespMapImpl<LS, LT>
    for RespMapCons<S, T, Tl>
where
    S: StatusCode,
    Tl: RespMap,
    LS: StatusCode,
{
    fn deserialize_body(s: &str) -> Result<LT, RespError> {
        Tl::deserialize_body::<LS, LT>(s)
    }
}

impl<S, T: for<'de> Deserialize<'de>, Tl> RespMapImpl<S, T>
    for RespMapCons<S, T, Tl>
where
    S: StatusCode,
    Tl: RespMap,
{
    fn deserialize_body(s: &str) -> Result<T, RespError> {
        ::serde_json::from_str(s).map_err(RespError::Json)
    }
}
