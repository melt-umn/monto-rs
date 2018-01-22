//! An HTTP client designed for Monto.

mod headers;

use std::collections::HashSet;
use std::marker::PhantomData;

use futures::{Async, Future};
use hyper::{Body, Request};
pub use hyper::Method;
use hyper::client::{Client as HyperClient, FutureResponse, HttpConnector};
use hyper::error::Error as HyperError;
use hyper::header::{Accept, ContentLength, ContentType};
use serde::{Deserialize, Serialize};
use tokio_core::reactor::Handle;
pub use url::Url;

use monto3_protocol::{ProtocolExtension, ProtocolVersion};

use self::headers::{MontoExtension, MontoVersion};

/// A simple HTTP client for Monto.
///
/// This client:
///
///  - Always sends and receives data as JSON, setting the `Accept`,
///    `Content-Length`, and `Content-Type` headers appropriately.
///  - Sends the `Monto-Version` header corresponding to the `ProtocolVersion`
///    provided at construction time or by the `set_version` method.
///  - Sends `Monto-Extension` headers corresponding to the enabled extensions,
///    as provided at construction time or by the `set_version` method.
pub struct HttpClient<E: ProtocolExtension> {
    base_url: Url,
    client: HyperClient<HttpConnector, Body>,
    extensions: HashSet<E>,
    version: ProtocolVersion,
}

impl<E: ProtocolExtension> HttpClient<E> {
    /// Creates a new `HttpClient`.
    ///
    /// Note: Panics if `base_url` is
    /// [`cannot_be_a_base`](../struct.Url.html#method.cannot_be_a_base).
    pub fn new<I: IntoIterator<Item = E>>(
        handle: &Handle,
        base_url: Url,
        extensions: I,
        version: ProtocolVersion,
    ) -> HttpClient<E> {
        assert!(!base_url.cannot_be_a_base());

        let client = HyperClient::new(handle);
        let extensions = extensions.into_iter().collect();
        HttpClient {
            base_url,
            client,
            extensions,
            version,
        }
    }

    /// Performs a request to the given path with the given method and body.
    /// The path will be relative to the `base_url` given on construction.
    pub fn request<T: Serialize, U: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        method: Method,
        body: &T,
    ) -> RequestFuture<U> {
        let url = self.base_url
            .join(path)
            .chain_err(|| {
                RequestErrorKind::BadPath(
                    self.base_url.clone(),
                    path.to_string(),
                )
            })
            .and_then(|url| {
                url.to_string().parse().chain_err(|| {
                    RequestErrorKind::BadPath(
                        self.base_url.clone(),
                        path.to_string(),
                    )
                })
            });
        let url = match url {
            Ok(url) => url,
            Err(err) => {
                return RequestError::with_chain(
                    err,
                    RequestErrorKind::BadPath(
                        self.base_url.clone(),
                        path.to_string(),
                    ),
                ).into()
            }
        };

        let mut req = Request::new(method, url);
        let body = match ::serde_json::to_vec(body) {
            Ok(body) => body,
            Err(err) => {
                return RequestError::with_chain(
                    err,
                    RequestErrorKind::CouldntSerialize,
                ).into()
            }
        };

        {
            // TODO: Remove this block once NLL is stable.
            let headers = req.headers_mut();
            headers.set(Accept::json());
            headers.set(ContentLength(body.len() as u64));
            headers.set(ContentType::json());
            headers.set(MontoExtension(self.extensions.clone()));
            headers.set(MontoVersion(self.version));
        }

        req.set_body(body);
        self.client.request(req).into()
    }
}

/// A Future for a request.
pub struct RequestFuture<T: for<'de> Deserialize<'de>> {
    inner: Option<RequestFutureInner>,
    _phantom: PhantomData<T>,
}

impl<T: for<'de> Deserialize<'de>> From<FutureResponse> for RequestFuture<T> {
    fn from(r: FutureResponse) -> RequestFuture<T> {
        RequestFuture {
            inner: Some(RequestFutureInner::Normal(r)),
            _phantom: PhantomData,
        }
    }
}

impl<E: Into<RequestError>, T: for<'de> Deserialize<'de>> From<E>
    for RequestFuture<T> {
    fn from(err: E) -> RequestFuture<T> {
        RequestFuture {
            inner: Some(RequestFutureInner::Error(err.into())),
            _phantom: PhantomData,
        }
    }
}

impl<T: for<'de> Deserialize<'de>> Future for RequestFuture<T> {
    type Item = T;
    type Error = RequestError;
    fn poll(&mut self) -> Result<Async<T>, RequestError> {
        let (inner, out) = match self.inner.take() {
            Some(RequestFutureInner::Normal(mut f)) => match f.poll() {
                Ok(Async::Ready(out)) => {
                    // TODO
                    unimplemented!("{:?}", out)
                },
                Ok(Async::NotReady) => (Some(RequestFutureInner::Normal(f)), Ok(Async::NotReady)),
                Err(err) => (None, Err(err.into())),
            },
            Some(RequestFutureInner::Error(err)) => (None, Err(err)),
            None => panic!("Called .poll() on a RequestFuture that already yielded a value!"),
        };
        self.inner = inner;
        out
    }
}

enum RequestFutureInner {
    Normal(FutureResponse),
    Error(RequestError),
}

error_chain! {
    types {
        RequestError, RequestErrorKind, RequestResultExt;
    }
    errors {
        /// A bad path was given as an argument.
        BadPath(base_url: Url, path: String) {
            description("An invalid path was used.")
            display("Can't join the path {:?} to the base URL {:?}", path, base_url)
        }

        /// The body couldn't be serialized.
        CouldntSerialize {
            description("The body couldn't be serialized.")
            display("The body couldn't be serialized.")
        }
    }
    foreign_links {
        Hyper(HyperError) #[doc = "An error from Hyper."];
    }
}
