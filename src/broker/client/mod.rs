//! The Client Protocol side of the Broker.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use either::{Left, Right};
use futures::{Async, Future, Poll, Stream};
use futures::future::{Empty, empty, err, ok};
use hyper::{Body, Method, Request, Response, StatusCode};
use hyper::Error as HyperError;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service};
use serde_json;
use tokio_core::net::{Incoming, TcpListener, TcpStream};
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
        match (method, uri.path()) {
            (Method::Post, "/monto/version") => {
                Box::new(json_request(body).and_then(|cn: ClientNegotiation| {
                    debug!("Got ClientNegotiation {:?}", cn);
                    let broker = self.0.borrow();
                    let cbn = ClientBrokerNegotiation {
                        monto: ProtocolVersion {
                            major: 3,
                            minor: 0,
                            patch: 0,
                        },
                        broker: broker.config.version.clone().into(),
                        extensions: broker.config.extensions.client.clone(),
                        services: broker.services.iter().map(|s| s.negotiation.clone()).collect(),
                    };
                    let status = StatusCode::NotImplemented;
                    json_response(cbn, status)
                }).or_else(|e| match e {
                    Left(e) => Box::new(err(e)),
                    Right(e) => error_response(StatusCode::InternalServerError),
                }))
            },
            (method, path) => {
                warn!("404 {} {}", method, path);
                Box::new(ok(Response::new()
                    .with_body("404 Not Found")
                    .with_header(ContentLength(13))
                    .with_header(ContentType("text/plain".parse().unwrap()))
                    .with_status(StatusCode::NotFound)))
            },
        }
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
