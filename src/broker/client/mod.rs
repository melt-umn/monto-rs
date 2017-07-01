//! The Client Protocol side of the Broker.
//!
//! TODO: This whole module needs a rusty axe and some lighter fluid applied to
//! it.

use std::cell::RefCell;
use std::rc::Rc;

use either::{Left, Right};
use futures::{Async, Future, Poll, Stream};
use futures::future::{Empty, empty, err};
use hyper::{Body, Method, Request, Response, StatusCode};
use hyper::Error as HyperError;
use hyper::server::{Http, Service};
use log::LogLevel;
use tokio_core::net::{Incoming, TcpListener};
use tokio_core::reactor::Handle;
use void::Void;

use broker::Broker;
use client::messages::{ClientNegotiation, ClientBrokerNegotiation};
use common::{error_response, json_request, json_response};
use common::messages::ProtocolVersion;

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

struct Client(Rc<RefCell<Broker>>);

impl Service for Client {
    type Request = Request;
    type Response = Response<Body>;
    type Error = HyperError;
    type Future = Box<Future<Item=Response<Body>, Error=HyperError>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, _, body) = req.deconstruct();
        Box::new(match (method.clone(), uri.path()) {
            (Method::Post, "/monto/version") => {
                // Make a reference to the Broker, which we move into the and_then closure.
                let broker = self.0.clone();

                // Deserialize the body, then...
                Box::new(json_request(body).and_then(move |cn: ClientNegotiation| {
                    // Log that we got the client negotiation message.
                    debug!("Got ClientNegotiation {:?}", cn);

                    // Build a response.
                    let cbn = {
                        let broker = broker.borrow();
                        ClientBrokerNegotiation {
                            monto: ProtocolVersion {
                                major: 3,
                                minor: 0,
                                patch: 0,
                            },
                            broker: broker.config.version.clone().into(),
                            extensions: broker.config.extensions.client.clone(),
                            services: broker.services.iter().map(|s| s.negotiation.clone()).collect(),
                        }
                    };

                    // Check for compatibility.
                    let status = if cbn.monto.compatible(&cn.monto) {
                        StatusCode::Ok
                    } else {
                        StatusCode::BadRequest
                    };

                    // Send the response.
                    json_response(cbn, status)
                }).or_else(|e| {
                    // Log the error.
                    error!("{}", e);

                    match e {
                        // If it's a Hyper error, just pass it along.
                        Left(e) => Box::new(err(e)),
                        // If it's serde's though, transform it into a 500.
                        Right(_) => error_response(StatusCode::InternalServerError)
                    }
                }))
            },
            _ => error_response(StatusCode::NotFound),
        }.map(move |r| {
            let status = r.status();
            let level = if status.is_server_error() || status.is_strange_status() {
                LogLevel::Error
            } else if status.is_client_error() {
                LogLevel::Warn
            } else {
                LogLevel::Info
            };
            log!(level, "{} {} {}", u16::from(r.status()), method, uri.path());
            r
        }))
    }
}

/// A Future for the Broker serving to Clients.
pub struct ServeFuture<F: Future> {
    broker: Rc<RefCell<Broker>>,
    handle: Handle,
    listener: Incoming,
    stop: F,
}

impl<F: Future> Future for ServeFuture<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.stop.poll() {
            Ok(Async::NotReady) => match self.listener.poll() {
                Ok(Async::Ready(Some((stream, remote)))) => {
                    info!("Got client connection from {}", remote);
                    let service = Client(self.broker.clone());
                    Http::new().bind_connection(&self.handle, stream, remote, service);
                    Ok(Async::NotReady)
                },
                Ok(Async::Ready(None)) => {
                    panic!("TcpListener.incoming() stream ended! (This is documented to be impossible)");
                },
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => {
                    error!("{}", err);
                    panic!("TODO proper error handling: {}", err);
                },
            },
            poll => poll,
        }
    }
}
