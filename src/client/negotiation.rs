use std::mem;

use either::{Either, Left, Right};
use futures::{Async, Future, Poll, Stream};
use hyper;
use hyper::{Body, StatusCode};
use hyper::client::FutureResponse;
use serde_json;
use url::{ParseError as UrlError, Url};

use super::{Client, HttpClient};

/// A Future for a Client negotiating version information and establishing a
/// connection to the Broker.
#[derive(Debug)]
pub struct Negotiation {
    inner: Result<NegotiationInner, Option<NegotiationError>>,
}

impl Negotiation {
    /// Creates a new instance of Negotiation.
    pub(crate) fn new(base_url: Url, client: HttpClient, future: FutureResponse) -> Negotiation {
        Negotiation {
            inner: Ok(NegotiationInner {
                url: base_url,
                client,
                state: Left(future),
            }),
        }
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
        println!("neg {:?}", self);
        match self.inner {
            Ok(ref mut future) => future.poll(),
            Err(ref mut err) => Err(err.take().unwrap()),
        }
    }
}

#[derive(Debug)]
struct NegotiationInner {
    url: Url,
    client: HttpClient,
    state: Either<FutureResponse, (StatusCode, Body, Vec<u8>)>,
}

impl Future for NegotiationInner {
    type Item = Client;
    type Error = NegotiationError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // State machines interact poorly with Rust's lifetime system. This may
        // be fixed in a future release. Alternately, it'd probably be fairly
        // easy to make a procedural macro that overloads the `become` syntax.
        //
        // As a result, this function is somewhat convoluted. When procedural
        // macros are stabilized, I'll probably give the state machine
        // generator a try; if it goes well, this whole module will probaby be
        // refactored.

        let (next_state, result) = match self.state {
            Left(ref mut future) => match future.poll() {
                Ok(Async::Ready(res)) => {
                    let next = Right((res.status(), res.body(), Vec::new()));
                    (next, Async::NotReady)
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(err) => return Err(err.into()),
            },
            Right(ref mut state) => match state.1.poll() {
                Ok(Async::Ready(Some(chunk))) => {
                    println!("chunk {:?}", chunk);
                    state.2.extend(chunk);
                    return Ok(Async::NotReady);
                },
                Ok(Async::Ready(None)) => {
                    let cbn = serde_json::from_slice(&state.2)?;
                    println!("{:?}", cbn);
                    unimplemented!()
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(err) => return Err(err.into()),
            },
        };
        println!("next {:?}", next_state);
        mem::replace(&mut self.state, next_state);
        Ok(result)
    }
}

error_chain! {
    types {
        NegotiationError, NegotiationErrorKind, NegotiationResultExt;
    }
    foreign_links {
        Hyper(hyper::Error)
            #[doc = "An error from the network."];
        Serde(serde_json::Error)
            #[doc = "An invalid response was received."];
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
