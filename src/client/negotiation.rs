use either::{Either, Left, Right};
use futures::{Async, Future, Poll};
use hyper;
use hyper::{Body, Response, StatusCode};
use hyper::client::FutureResponse;
use url::{ParseError as UrlError, Url};

use common::json_request;
use super::{Client, HttpClient};

/// A Future for a Client negotiating version information and establishing a
/// connection to the Broker.
pub struct Negotiation {
    inner: Result<NegotiationInner, Option<NegotiationError>>,
}

impl Negotiation {
    /// Creates a new instance of Negotiation.
    pub(crate) fn new(base_url: Url, http: HttpClient, f: FutureResponse) -> Negotiation {
        unimplemented!()
    }

    /// Creates a new Negotiation that immediately resolves to an error.
    pub(crate) fn err(err: NegotiationError) -> Negotiation {
        Negotiation {
            inner: Err(Some(err)),
        }
    }
}

impl Future for Negotiation {
    type Item = Client;
    type Error = NegotiationError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner {
            Ok(ref mut future) => future.poll(),
            Err(ref mut err) => Err(err.take().unwrap()),
        }
    }
}

struct NegotiationInner {
    url: Url,
    client: HttpClient,
    state: Either<FutureResponse, (StatusCode, Body, Vec<u8>)>,
}

impl Future for NegotiationInner {
    type Item = Client;
    type Error = NegotiationError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.state {
            Left(ref mut future) => match future.poll() {
                Ok(Async::Ready(res)) => {
                    // This ought to be safe, I think.
                    // Do I need to use {,Ref,Unsafe}Cell?

                    // self.state = Right((res.status(), res.body(), Vec::new()));
                    Ok(Async::NotReady)
                },
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(err.into()),
            },
            Right(ref mut state) => {
                // let (status, body, buf) = state;
                unimplemented!()
            },
        }
    }
}

error_chain! {
    types {
        NegotiationError, NegotiationErrorKind, NegotiationResultExt;
    }
    foreign_links {
        Hyper(hyper::Error)
            #[doc = "An error from the network."];
    }
    errors {
        /// A status other than Ok was received from the Broker, indicating
        /// that the Client is not compatible.
        BadStatus(code: StatusCode) {
            description("The Broker and Client are not compatible")
            display("The Broker and Client are not compatible: got {} from the Broker", code)
        }

        /// The given config had an invalid broker location specified.
        BadConfigURL(err: UrlError) {
            description("The config was invalid")
        }
    }
}
