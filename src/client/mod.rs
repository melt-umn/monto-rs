//! An implementation of a Monto client.

use futures::{Async, Future, Poll};
use hyper::Method::Post;
use hyper::client::*;
use hyper::client::Client as HttpClient;
use hyper::error::Error as HyperError;
use hyper::error::UriError;
use hyper::header::{ContentLength, ContentType};
use serde_json;
use std::collections::BTreeSet;
use tokio_core::reactor::Handle;
use types::{ClientExtension, ClientNegotiation, ClientVersion, Identifier, MontoVersion, ServiceNegotiation};

error_chain! {
    foreign_links {
        Hyper(HyperError);
        Json(serde_json::Error);
        InvalidUri(UriError);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// The address to connect to. This should be the URI Authority.
    pub broker_addr: String,

    /// The client version information to send in version negotiation. The
    /// default is the crate's own version information.
    pub client_version: ClientVersion,

    /// Whether TLS should be used.
    pub use_tls: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            broker_addr: "localhost:28888".to_owned(),
            client_version: ClientVersion {
                id: Identifier("edu.umn.cs.melt.monto3_rs.client".to_owned()),
                name: None,
                vendor: None,
                major: 0,
                minor: 1,
                patch: 0,
            },
            use_tls: false,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    config: Config,
    extensions: BTreeSet<ClientExtension>,
    http: HttpClient<HttpConnector>,
}

impl Client {
    /// Creates a new Client.
    pub fn new(config: Config, handle: Handle) -> NewClientFuture {
        let client = HttpClient::new(&handle);
        match Client::negotiation_request(&config, BTreeSet::new()) {
            Ok(req) => {
                let res = client.request(req);
                NewClientFuture {
                    client: Some(client),
                    res: Ok(res),
                }
            },
            Err(err) => NewClientFuture {
                client: None,
                res: Err(err),
            },
        }
    }

    /// Returns the extensions enabled on the connection.
    pub fn extensions(&self) -> &BTreeSet<ClientExtension> {
        unimplemented!()
    }

    /// Returns the services present on the Broker.
    pub fn services(&self) -> &BTreeSet<ServiceNegotiation> {
        unimplemented!()
    }

    /// Creates a Request to use for negotiation.
    fn negotiation_request(config: &Config, extensions: BTreeSet<ClientExtension>) -> Result<Request> {
        // Build the negotiation request URI.
        // TODO Refactor when hyperium/hyper#1102 is fixed.
        let scheme = if config.use_tls {
            "https"
        } else {
            "http"
        };
        let uri = format!("{}://{}/monto/version", scheme, config.broker_addr);
        println!("{}", uri);
        let uri = uri.parse()?;

        // Create the Message.
        let msg = ClientNegotiation {
            client: config.client_version.clone(),
            monto: MontoVersion::default(),
            extensions: extensions,
        };
        let msg = serde_json::to_string(&msg)?;

        // Create and return the Request.
        let mut req = Request::new(Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(msg.len() as u64));
        req.set_body(msg);
        Ok(req)
    }
}

pub struct NewClientFuture {
    client: Option<HttpClient<HttpConnector>>,
    res: Result<FutureResponse>,
}

impl Future for NewClientFuture {
    type Item = Client;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.res.as_mut() {
            Ok(mut res) => match res.poll() {
                Ok(Async::Ready(res)) => unimplemented!(), // TODO
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(err.into()),
            },
            Err(err) => unimplemented!(),
        }
    }
}
