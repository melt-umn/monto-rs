//! The Client Protocol side of the Broker.

use broker::Broker;
use client::messages::ClientBrokerNegotiation;
use common::messages::ProtocolVersion;
use futures::{Async, Future, Poll, Stream};
use futures::future::{Empty, empty};
use hyper::{Body, Request, Response};
use hyper::Error as HyperError;
use hyper::server::{Http, Service};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio_core::net::{Incoming, TcpListener, TcpStream};
use tokio_core::reactor::Handle;
use void::Void;

impl Broker {
    /// Returns the ClientBrokerNegotiation that the Broker returns to Clients.
    pub fn client_negotiation(&self) -> ClientBrokerNegotiation {
        ClientBrokerNegotiation {
            monto: ProtocolVersion {
                major: 3,
                minor: 0,
                patch: 0,
            },
            broker: self.config.version.clone().into(),
            extensions: self.config.extensions.client.clone(),
            services: self.services.iter().map(|s| s.negotiation.clone()).collect(),
        }
    }

    /// Returns a Future that will resolve once the given Future resolves,
    /// serving clients until then.
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
        info!("Got request {:?}", req);
        unimplemented!()
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
                    error!("TcpListener.incoming() stream ended! (This is documented to be impossible)");
                    unreachable!();
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
