use std::mem;

use either::{Either, Left, Right};
use futures::{Async, Future, Poll, Stream};
use futures::stream::Concat2;
use hyper;
use hyper::{Body, StatusCode};
use hyper::client::FutureResponse;
use serde_json;
use url::{ParseError as UrlError, Url};

use super::{Client, HttpClient};
use super::messages::{ClientNegotiation, ClientBrokerNegotiation};

/// A Future for a Client negotiating version information and establishing a
/// connection to the Broker.
pub struct Negotiation {
    inner: NegotiationInner,
}

impl Negotiation {
    /// Creates a new instance of Negotiation.
    pub(crate) fn new(base_url: Url, client: HttpClient, cn: ClientNegotiation, future: FutureResponse) -> Negotiation {
        let inner = future.map_err(NegotiationError::from)
            .and_then(|res| res.body().concat2().map_err(NegotiationError::from))
            .and_then(|body| serde_json::from_slice(body.as_ref()).map_err(NegotiationError::from))
            .and_then(|cbn| Negotiation::negotiate(base_url, client, cn, cbn));
        Negotiation { inner: Box::new(inner) }
    }

    /// Creates a new instance of Negotiation that immediately returns an error.
    pub(crate) fn err(err: NegotiationError) -> Negotiation {
        unimplemented!()
    }

    fn negotiate(base_url: Url, http: HttpClient, cn: ClientNegotiation, cbn: ClientBrokerNegotiation) -> Result<Client, NegotiationError> {
        if cn.monto.compatible(&cbn.monto) {
            let services = cbn.services.into_iter()
                .map(|sn| (sn.service.id, sn.products))
                .collect();
            Ok(Client { base_url, http, services })
        } else {
            unimplemented!()
        }
    }
}

impl Future for Negotiation {
    type Item = Client;
    type Error = NegotiationError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

type NegotiationInner = Box<Future<Item=Client, Error=NegotiationError>>;

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
