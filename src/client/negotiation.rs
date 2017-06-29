use either::Left;
use futures::{Async, Future, Poll};
use hyper;
use hyper::StatusCode;
use hyper::client::FutureResponse;
use url::{ParseError as UrlError, Url};

use super::{Client, HttpClient};

/// Handles negotiating a protocol version and extensions with the Broker.
pub fn negotiate(base_url: Url, http: HttpClient, f: FutureResponse) -> Box<Future<Item=Client, Error=NegotiationError>> {
    Box::new(f.map_err(Left)
        .map(Response::Body)
        .and_then(json_request))
}

/// A Future for a Client negotiating version information and establishing a
/// connection to the Broker.
pub struct Negotiation(Url, HttpClient, FutureResponse);

impl Negotiation {
    pub(crate) fn new(url: Url, client: HttpClient, res: FutureResponse) -> Negotiation {
        Negotiation(url, client, res)
    }
}

impl Future for Negotiation {
    type Item = Client;
    type Error = NegotiationError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.2.poll() {
            Ok(Async::Ready(res)) => match res.status() {
                StatusCode::Ok => unimplemented!(),
                code => Err(NegotiationErrorKind::BadStatus(code).into()),
            },
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err.into()),
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
