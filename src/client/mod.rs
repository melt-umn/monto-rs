//! Functions and types useful for implementing the Client Protocol, as defined
//! in
//! [Section 4](https://melt-umn.github.io/monto-v3-draft/draft02/#4-the-client-protocol)
//! of the specification.

pub mod messages;
mod negotiation;

use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

use futures::{Async, Future, Poll};
use futures::future::err;
use hyper;
use hyper::{Get, Post, Put, Request, StatusCode, Uri};
use hyper::client::FutureResponse;
use hyper::header::{ContentLength, ContentType};
use serde_json;
use tokio_core::reactor::Handle;
use url::{ParseError, Url};

use common::messages::{Identifier, Language, Product, ProductIdentifier, ProductName, ProductValue, ProtocolVersion, SoftwareVersion};
use self::messages::{BrokerGetError, BrokerPutError, ClientBrokerNegotiation, ClientNegotiation};
pub use self::negotiation::{NegotiationError, NegotiationErrorKind};

type HttpClient = hyper::client::Client<hyper::client::HttpConnector>;

/// A Monto Client.
pub struct Client {
    base_url: Url,
    http: HttpClient,
    services: BTreeMap<Identifier, BTreeSet<ProductIdentifier>>,
}

impl Client {
    /// Builds a Monto URI.
    ///
    /// TODO: This can be made more efficient when
    /// [hyperium/hyper#1102](https://github.com/hyperium/hyper/issues/1102) is
    /// fixed.
    fn make_uri(&self, service: Option<&Identifier>, product: &ProductName, language: Option<&Language>, path: &str) -> Uri {
        let url = match service {
            Some(service) => self.base_url.join(&service.to_string()),
            None => self.base_url.join("broker"),
        }.expect("Illegal internal Client state -- base_url is cannot-be-a-base");
        unimplemented!()
    }

    /// Creates a new Client running on the given event loop with the given
    /// configuration, as specified in Sections
    /// [4.1](https://melt-umn.github.io/monto-v3-draft/draft02/#4-1-connection-initiation)
    /// and
    /// [4.2](https://melt-umn.github.io/monto-v3-draft/draft02/#4-2-version-negotiation)
    /// of the specification.
    pub fn new(config: Config, handle: Handle) -> Box<Future<Item=Client, Error=NegotiationError>> {
        let scheme = "http"; // TODO TLS support.

        let base_url = format!("{}://{}:{}/monto/", scheme, config.host, config.port);
        let mut base_url = match Url::parse(&base_url) {
            Ok(url) => url,
            Err(e) => return Box::new(err(NegotiationErrorKind::BadConfigURL(e).into())),
        };
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }

        let body = serde_json::to_string(&ClientNegotiation {
            monto: ProtocolVersion {
                major: 3,
                minor: 0,
                patch: 0,
            },
            client: config.version,
            extensions: BTreeSet::new(),
        }).unwrap();

        let url = match base_url.join("version") {
            Ok(url) => url,
            Err(e) => return Box::new(err(NegotiationErrorKind::BadConfigURL(e).into())),
        };
        let mut req = Request::new(Post, url.to_string().parse().unwrap());
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(body.len() as u64));
        req.set_body(body);

        let http = HttpClient::new(&handle);
        let future = http.request(req);
        negotiation::negotiate(base_url, http, future)
    }

    /// Attempts to retrieve a Product from the Broker, as described in
    /// [Section 4.4](https://melt-umn.github.io/monto-v3-draft/draft02/#4-4-requesting-products)
    /// of the specification.
    pub fn request<P: ProductValue>(&mut self, service: &Identifier, p: &ProductIdentifier) -> RequestFuture<P> {
        let req = Request::new(Get, self.make_uri(Some(service), &p.name, Some(&p.language), &p.path));
        RequestFuture::new(self.http.request(req))
    }

    /// Returns an iterator over the Products that can be requested by the Client.
    pub fn products(&self) -> ProductsIter {
        let iter = self.services.iter().flat_map(|(service, products)| {
            products.iter().map(move |product| (service, product))
        });
        ProductsIter(Box::new(iter))
    }

    /// Sends a Product to the Broker, as described in
    /// [Section 4.3](https://melt-umn.github.io/monto-v3-draft/draft02/#4-3-sending-products)
    /// of the specification.
    pub fn send_product<P: ProductValue>(&mut self, p: &Product<P>) -> SendFuture {
        // let mut req = Request::new(Put, self.make_uri(None, &p.name, Some(&p.language), &p.path));
        unimplemented!()
    }
}

/// Configuration for a Client.
pub struct Config {
    /// The host to connect to the Broker on.
    ///
    /// Defaults to `localhost`, as per
    /// [Section 4.1](https://melt-umn.github.io/monto-v3-draft/draft02/#4-1-connection-initiation)
    /// of the specification.
    pub host: String,

    /// The port to connect to the Broker on.
    ///
    /// Defaults to `28888`, as per
    /// [Section 4.1](https://melt-umn.github.io/monto-v3-draft/draft02/#4-1-connection-initiation)
    /// of the specification.
    pub port: u16,

    /// The name and version of the client.
    pub version: SoftwareVersion,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: "localhost".to_owned(),
            port: 28888,
            version: SoftwareVersion {
                id: "edu.umn.cs.melt.monto_rs.client".parse().unwrap(),
                name: None,
                vendor: None,
                major: 0,
                minor: 1,
                patch: 0,
            },
        }
    }
}

/// An iterator over the products a Client can request.
///
/// TODO: This can be made more efficient when
/// [`conservative_impl_trait`](https://github.com/rust-lang/rust/issues/34511)
/// is stabilized.
pub struct ProductsIter<'a>(Box<Iterator<Item=(&'a Identifier, &'a ProductIdentifier)> + 'a>); // TODO Don't use a trait object.

impl<'a> Iterator for ProductsIter<'a> {
    type Item = (&'a Identifier, &'a ProductIdentifier);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// A Future for a Product being requested from the Broker.
pub struct RequestFuture<P> {
    future: FutureResponse,
    _phantom: PhantomData<P>,
}

impl<P: ProductValue> RequestFuture<P> {
    fn new(f: FutureResponse) -> RequestFuture<P> {
        RequestFuture {
            future: f,
            _phantom: PhantomData,
        }
    }
}

impl<P: ProductValue> Future for RequestFuture<P> {
    type Item = Product<P>;
    type Error = BrokerGetError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}

/// A Future for a Product being sent to the Broker.
pub struct SendFuture();

impl Future for SendFuture {
    type Item = ();
    type Error = BrokerPutError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}
