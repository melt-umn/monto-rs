//! The Client Protocol side of the Broker.
//!
//! TODO: This whole module needs a rusty axe and some lighter fluid applied to
//! it.

mod negotiation;
mod req_products;
mod send_products;

use std::cell::RefCell;
use std::rc::Rc;

use either::{Either, Left, Right};
use futures::{Async, Future, Poll, Stream};
use futures::future::{empty, err, Empty};
use hyper::{Body, Error as HyperError, Method, Request, Response, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Http, Service};
use log::LogLevel;
use mime;
use serde_json::{Error as JsonError, Value};
use tokio_core::net::{Incoming, TcpListener};
use tokio_core::reactor::Handle;
use url::form_urlencoded::parse as parse_query;
use void::Void;

use monto3_common::{error_response, json_request};
use monto3_common::messages::{Language, ProductIdentifier, ProductName};

use Broker;

type BoxedFuture = Box<Future<Item = Response, Error = Either<HyperError, JsonError>>>;

impl Broker {
    /// Returns a Future that will resolve once the given Future resolves,
    /// serving clients until then.
    ///
    /// TODO: This can be made more efficient when
    /// [`conservative_impl_trait`](https://github.com/rust-lang/rust/issues/34511)
    /// is stabilized.
    pub fn serve_until<F: Future>(self, stop: F) -> ServeFuture<F> {
        let listener = TcpListener::bind(&self.config.net.addr, &self.handle)
            .expect("TODO proper error handling")
            .incoming();
        let handle = self.handle.clone();
        let broker = Rc::new(RefCell::new(self));
        ServeFuture {
            broker,
            handle,
            http: Http::new(),
            listener,
            stop,
        }
    }

    /// Returns a Future that will never resolve, but will serves clients forever.
    ///
    /// TODO: This can be made more efficient when
    /// [`conservative_impl_trait`](https://github.com/rust-lang/rust/issues/34511)
    /// is stabilized.
    pub fn serve_forever(self) -> ServeFuture<Empty<Void, Void>> {
        self.serve_until(empty())
    }
}

#[derive(Clone)]
pub(crate) struct Client(pub Rc<RefCell<Broker>>);

impl Service for Client {
    type Request = Request;
    type Response = Response<Body>;
    type Error = HyperError;
    type Future = Box<Future<Item = Response<Body>, Error = HyperError>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, headers, body) = req.deconstruct();
        let path_str = uri.path().to_string();
        let mut query_pairs = parse_query(uri.query().unwrap_or("").as_bytes());
        let path = uri.path().split("/").collect::<Vec<_>>();
        let f: BoxedFuture = match (method.clone(), &path) {
            (Method::Post, path) if path == &["", "monto", "version"] => {
                let client = self.clone();
                Box::new(json_request(body).and_then(move |cn| client.negotiation(cn)))
            }
            (Method::Put, path)
                if path.len() == 4 && path[0] == "" && path[1] == "monto"
                    && path[2] == "broker" =>
            {
                let pt = path[3].parse().expect("TODO Error handling");
                let pp = query_pairs
                    .find(|&(ref k, _)| k == "path")
                    .map(|(_, v)| v.into_owned())
                    .expect("TODO Error handling");
                let language = query_pairs
                    .find(|&(ref k, _)| k == "language")
                    .map(|(_, v)| v.into_owned())
                    .map(Language::from);
                let client = self.clone();
                let ContentType(content_type) = headers
                    .get()
                    .map(Clone::clone)
                    .unwrap_or_else(ContentType::json);
                match (content_type.type_(), content_type.subtype()) {
                    (mime::TEXT, mime::PLAIN) => if pt == ProductName::Source {
                        Box::new(body.concat2().map_err(Left).and_then(move |b| {
                            let b = String::from_utf8_lossy(b.as_ref()).into_owned();
                            client.send_products(pt, pp, language, Value::String(b))
                        }))
                    } else {
                        panic!("TODO Error Handling");
                    },
                    (mime::APPLICATION, mime::JSON) => Box::new(
                        json_request(body)
                            .and_then(move |p| client.send_products(pt, pp, language, p)),
                    ),
                    _ => unimplemented!(),
                }
            }
            (Method::Get, path) if path.len() == 4 && path[0] == "" && path[1] == "monto" => {
                let service_id = path[2].parse().expect("TODO Error handling");
                let product_type = path[3].parse().expect("TODO Error handling");
                let product_path = query_pairs
                    .find(|&(ref k, _)| k == "path")
                    .map(|(_, v)| v.into_owned())
                    .expect("TODO Error handling");
                let language = query_pairs
                    .find(|&(ref k, _)| k == "language")
                    .map(|(_, v)| v.into_owned())
                    .map(Language::from)
                    .expect("TODO Error handling");
                Box::new(self.clone().req_products(
                    service_id,
                    ProductIdentifier {
                        language,
                        name: product_type,
                        path: product_path,
                    },
                ))
            }
            _ => Box::new(error_response(StatusCode::NotFound).map_err(Left)),
        };
        Box::new(
            f.or_else(|e| {
                // Log the error.
                error!("{}", e);

                match e {
                    // If it's a Hyper error, just pass it along.
                    Left(e) => Box::new(err(e)),
                    // If it's serde's though, transform it into a 500.
                    Right(_) => error_response(StatusCode::InternalServerError),
                }
            }).map(move |r| {
                    let status = r.status();
                    let level = if status.is_server_error() || status.is_strange_status() {
                        LogLevel::Error
                    } else if status.is_client_error() {
                        LogLevel::Warn
                    } else {
                        LogLevel::Info
                    };
                    log!(level, "{} {} {}", u16::from(r.status()), method, path_str);
                    r
                }),
        )
    }
}

/// A Future for the Broker serving to Clients.
pub struct ServeFuture<F: Future> {
    broker: Rc<RefCell<Broker>>,
    handle: Handle,
    http: Http,
    listener: Incoming,
    stop: F,
}

impl<F: Future> Future for ServeFuture<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.stop.poll() {
            Ok(Async::NotReady) => loop {
                match self.listener.poll() {
                    Ok(Async::Ready(Some((stream, remote)))) => {
                        info!("Got client connection from {}", remote);
                        let service = Client(self.broker.clone());
                        self.http
                            .bind_connection(&self.handle, stream, remote, service);
                    }
                    Ok(Async::Ready(None)) => {
                        panic!(
                                "TcpListener.incoming() stream ended! (This is documented to be impossible)"
                            );
                    }
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(err) => {
                        error!("{}", err);
                        panic!("TODO proper error handling: {}", err);
                    }
                }
            },
            poll => poll,
        }
    }
}
