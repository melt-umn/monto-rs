//! Functions and types useful for implementing the Client Protocol, as defined
//! in
//! [Section 4](https://melt-umn.github.io/monto-v3-draft/draft02/#4-the-client-protocol)
//! of the specification.

pub mod messages;
mod negotiation;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::marker::PhantomData;
use std::io::Read;
use std::path::{Path, PathBuf};

use either::Either;
use futures::{Future, Poll, Stream};
use futures::future::{err, result};
use hyper;
use hyper::{Get, Post, Put, Request, StatusCode, Uri};
use hyper::client::FutureResponse;
use hyper::header::{ContentLength, ContentType};
use serde_json;
use tokio_core::reactor::Handle;
use url::Url;

use common::messages::{GenericProduct, Identifier, Language, Product, ProductDescriptor, ProductIdentifier, ProductName, ProtocolVersion, SoftwareVersion};
use common::products::Source;
use self::messages::{BrokerGetError, BrokerPutError, ClientNegotiation};
pub use self::negotiation::{Negotiation, NegotiationError, NegotiationErrorKind};

type HttpClient = hyper::client::Client<hyper::client::HttpConnector>;

/// A Monto Client.
pub struct Client {
    base_url: Url,
    http: HttpClient,
    services: BTreeMap<Identifier, BTreeSet<ProductDescriptor>>,
}

impl Client {
    /// Builds a Monto URI.
    ///
    /// TODO: This can be made more efficient when
    /// [hyperium/hyper#1102](https://github.com/hyperium/hyper/issues/1102) is
    /// fixed.
    fn make_uri(&self, service: Option<&Identifier>, product: &ProductName, language: Option<&Language>, path: &str) -> Uri {
        let mut url = match service {
            Some(service) => self.base_url.join(&format!("{}/", service)),
            None => self.base_url.join("broker/"),
        }.and_then(|url| {
            url.join(&product.to_string())
        }).expect("Illegal internal Client state -- base_url is cannot-be-a-base");

        url.query_pairs_mut().append_pair("path", path);
        if let Some(language) = language {
            url.query_pairs_mut().append_pair("language", &language.to_string());
        }
        url.into_string().parse().unwrap()
    }

    /// Creates a new Client running on the given event loop with the given
    /// configuration, as specified in Sections
    /// [4.1](https://melt-umn.github.io/monto-v3-draft/draft02/#4-1-connection-initiation)
    /// and
    /// [4.2](https://melt-umn.github.io/monto-v3-draft/draft02/#4-2-version-negotiation)
    /// of the specification.
    pub fn new(config: Config, handle: Handle) -> Negotiation {
        let scheme = "http"; // TODO TLS support.

        let base_url = format!("{}://{}:{}/monto/", scheme, config.host, config.port);
        let mut base_url = match Url::parse(&base_url) {
            Ok(url) => url,
            Err(e) => return Negotiation::err(NegotiationErrorKind::BadConfigURL(e).into()),
        };
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }
        debug!("base_url is {}", base_url);

        let cn = ClientNegotiation {
            monto: ProtocolVersion {
                major: 3,
                minor: 0,
                patch: 0,
            },
            client: config.version,
            extensions: BTreeSet::new(),
        };
        let body = serde_json::to_string(&cn).unwrap();

        let url = match base_url.join("version") {
            Ok(url) => url,
            Err(e) => return Negotiation::err(NegotiationErrorKind::BadConfigURL(e).into()),
        };
        let mut req = Request::new(Post, url.to_string().parse().unwrap());
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(body.len() as u64));
        req.set_body(body);

        let http = HttpClient::new(&handle);
        let future = http.request(req);
        Negotiation::new(base_url, http, cn, future)
    }

    /// Attempts to retrieve a Product from the Broker, as described in
    /// [Section 4.4](https://melt-umn.github.io/monto-v3-draft/draft02/#4-4-requesting-products)
    /// of the specification.
    pub fn request<P: Product + 'static>(&mut self, service: &Identifier, p: &ProductIdentifier) -> Box<Future<Item=P, Error=RequestError>> {
        let path: &Path = p.path.as_ref();
        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            match path.canonicalize() {
                Ok(path) => path,
                Err(e) => return Box::new(err(e.into())),
            }
        };
        let path = path.display().to_string();

        let req = Request::new(Get, self.make_uri(Some(service), &p.name, Some(&p.language), &path));
        info!("Requesting product {:?} from {}", p, service);
        Box::new(self.http.request(req)
            .map_err(RequestError::from)
            .and_then(|res| {
                let status = res.status();
                res.body()
                    .concat2()
                    .map(move |b| (b, status))
                    .map_err(RequestError::from)
            })
            .and_then(|(body, status)| {
                result(match status {
                    StatusCode::Ok => {
                        serde_json::from_slice(body.as_ref())
                            .and_then(|gp: GenericProduct| P::from_json(gp.name, gp.language, gp.path, gp.value))
                            .map_err(RequestError::from)
                    },
                    _ => {
                        let e = RequestError::from(match serde_json::from_slice(body.as_ref()) {
                            Ok(bge) => RequestErrorKind::Broker(bge),
                            Err(err) => RequestErrorKind::Json(err),
                        });
                        Err(e)
                    },
                })
            }))
    }

    /// Returns an iterator over the Products that can be requested by the Client.
    pub fn products(&self) -> ProductsIter {
        let iter = self.services.iter().flat_map(|(service, products)| {
            products.iter().map(move |product| (service, product))
        });
        ProductsIter(Box::new(iter))
    }

    /// Sends a `source` Product to the Broker.
    pub fn send_file<P: AsRef<Path>>(&mut self, path: P, language: Language) -> Box<Future<Item=(), Error=SendError>> {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            match path.canonicalize() {
                Ok(path) => path,
                Err(e) => return Box::new(err(e.into())),
            }
        };

        let src = match File::open(&path) {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)
                    .map(|_| buf)
            },
            Err(e) => Err(e),
        };
        let src = match src {
            Ok(src) => src,
            Err(e) => return Box::new(err(e.into())),
        };
        self.send_product(&Source {
            contents: src,
            language,
            path: path.display().to_string(),
        })
    }

    /// Sends a Product to the Broker, as described in
    /// [Section 4.3](https://melt-umn.github.io/monto-v3-draft/draft02/#4-3-sending-products)
    /// of the specification.
    pub fn send_product<P: Product>(&mut self, p: &P) -> Box<Future<Item=(), Error=SendError>> {
        let path = PathBuf::from(p.path());
        let path = if path.is_absolute() {
            path
        } else {
            match path.canonicalize() {
                Ok(path) => path,
                Err(e) => return Box::new(err(e.into())),
            }
        };
        let path = path.display().to_string();

        let body = match serde_json::to_string(&p.value()) {
            Ok(body) => body,
            Err(e) => return Box::new(err(SendError::from(e))),
        };
        let mut req = Request::new(Put, self.make_uri(None, &p.name(), Some(&p.language()), &path));
        {
            let headers = req.headers_mut();
            headers.set(ContentLength(body.len() as u64));
            headers.set(ContentType::json());
        }
        req.set_body(body);
        Box::new(self.http.request(req).and_then(|r| {
            let status = r.status();
            r.body()
                .concat2()
                .map(move |b| (b, status))
        }).map_err(SendError::from).and_then(|(body, status)| {
            result(match status {
                StatusCode::NoContent => Ok(()),
                StatusCode::BadRequest => Err(match serde_json::from_slice(body.as_ref()) {
                    Ok(bpe) => SendErrorKind::Broker(bpe).into(),
                    Err(err) => SendError::from(err),
                }),
                _ => panic!("TODO Proper error handling"),
            })
        }))
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
pub struct ProductsIter<'a>(Box<Iterator<Item=(&'a Identifier, &'a ProductDescriptor)> + 'a>); // TODO Don't use a trait object.

impl<'a> Iterator for ProductsIter<'a> {
    type Item = (&'a Identifier, &'a ProductDescriptor);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

error_chain! {
    types {
        RequestError, RequestErrorKind, RequestResultExt;
    }
    foreign_links {
        Broker(BrokerGetError)
            #[doc = "An error from the Broker."];
        Hyper(::hyper::Error)
            #[doc = "An error connecting to the Broker."];
        Io(::std::io::Error)
            #[doc = "An I/O error."];
        Json(serde_json::Error)
            #[doc = "An invalid response (bad JSON) was received from the Broker."];
    }
}

error_chain! {
    types {
        SendError, SendErrorKind, SendResultExt;
    }
    foreign_links {
        Broker(BrokerPutError)
            #[doc = "An error from the Broker."];
        Hyper(::hyper::Error)
            #[doc = "An error connecting to the Broker."];
        Io(::std::io::Error)
            #[doc = "An I/O error."];
        Json(serde_json::Error)
            #[doc = "An invalid response (bad JSON) was received from the Broker."];
    }
}
