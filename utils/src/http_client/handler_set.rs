use std::marker::PhantomData;

use either::Either;
use serde::Deserialize;
use serde_json::Error as JsonError;
use void::Void;

/// A trait for representing HTTP status codes at the type level.
pub trait StatusCode {
    /// The value-level status code corresponding to this type-level status
    /// code.
    const VALUE: ::hyper::StatusCode;
}

/// A trait for a type-level map between response codes and structures to
/// deserialize response bodies to.
///
/// If this looks incredibly strange and unfamiliar, look at [the Haskell HList
/// package](https://hackage.haskell.org/package/HList) or [Strongly Typed
/// Heterogeneous Collections](http://okmij.org/ftp/Haskell/HList-ext.pdf).
pub trait HandlerSet<S: StatusCode, T: for<'de> Deserialize<'de>, E> {
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body(s: &str) -> Result<T, Either<JsonError, E>>;
}

/// A default handler at the end of a `HandlerSet`.
pub struct HandlerDefault<T: for<'de> Deserialize<'de>>(PhantomData<T>, Void);

impl<S, T, E> HandlerSet<S, T, E> for HandlerDefault<T>
where
    S: StatusCode,
    T: for<'de> Deserialize<'de>,
{
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body(s: &str) -> Result<T, Either<JsonError, E>> {
        unimplemented!()
    }
}

/// A status-handler pair placed in front of another `HandlerSet`.
pub struct HandlerCons<
    S: StatusCode,
    T: for<'de> Deserialize<'de>,
    E,
    Tl: HandlerSet<TlS, TlTy, E>,
    TlS: StatusCode,
    TlTy: for<'de> Deserialize<'de>,
>(PhantomData<(S, T, E, Tl, TlS, TlTy)>, Void);

impl<S, T, E, Tl, TlS, TlTy> HandlerSet<S, T, E>
    for HandlerCons<S, T, E, Tl, TlS, TlTy>
where
    S: StatusCode,
    T: for<'de> Deserialize<'de>,
    Tl: HandlerSet<TlS, TlTy, E>,
    TlS: StatusCode,
    TlTy: for<'de> Deserialize<'de>,
{
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body(s: &str) -> Result<T, Either<JsonError, E>> {
        unimplemented!()
    }
}

impl<S, T, E, Tl, TlS, TlTy> HandlerSet<TlS, T, E>
    for HandlerCons<S, T, E, Tl, TlS, TlTy>
where
    S: StatusCode,
    T: for<'de> Deserialize<'de>,
    Tl: HandlerSet<TlS, TlTy, E>,
    TlS: StatusCode,
    TlTy: for<'de> Deserialize<'de>,
{
    /// Attempts to deserialize the given JSON string into a body.
    fn deserialize_body(s: &str) -> Result<T, Either<JsonError, E>> {
        unimplemented!()
    }
}
