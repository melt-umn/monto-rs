//! The Client Protocol side of the Broker.

use either::{Either, Left, Right};
use futures::{Async, Future, Poll};
use futures::future::{Empty, empty};
use hyper::{Body, Request, Response};
use hyper::Error as HyperError;
use hyper::server::{Http, Service};
use monto::client::messages::ClientBrokerNegotiation;
use monto::common::messages::ProtocolVersion;
use std::sync::{Arc, Mutex};
use super::Broker;
use void::{Void, unreachable};

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

    /// Returns a Future, which will never resolve except with an error. During this Future's
    /// execution, the Broker will respond to Clients.
    pub fn serve_forever(self) -> ServeFuture<Empty<Void, Void>> {
        self.serve_until(empty())
    }

    /// Returns a Future, which will resolve when the given Future resolves or when an error is
    /// encountered. During the returned Future's execution, the Broker will respond to Clients.
    pub fn serve_until<F: Future>(self, stop: F) -> ServeFuture<F> {
        let broker = Arc::new(Mutex::new(self));
        let res = Http::new().bind(&self.config.net.addr, || Ok(Client(broker.clone())));
        // TODO Right now, this blocks...
        res.unwrap().run().unwrap();
        unimplemented!()
    }
}

struct Client(Arc<Mutex<Broker>>);

impl Service for Client {
    type Request = Request;
    type Response = Response<Body>;
    type Error = HyperError;
    type Future = Box<Future<Item=Response<Body>, Error=HyperError>>;

    fn call(&self, req: Request) -> Self::Future {
        unimplemented!()
    }
}

/// A Future for the Broker serving to Clients.
pub struct ServeFuture<F>
    where F: Future
{
    server: Box<Future<Item=Void, Error=HyperError>>,
    stop: F,
}

impl<F: Future> Future for ServeFuture<F> {
    type Item = F::Item;
    type Error = Either<F::Error, HyperError>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.stop.poll() {
            Ok(Async::Ready(val)) => Ok(Async::Ready(val)),
            Ok(Async::NotReady) => match self.server.poll() {
                Ok(Async::Ready(void)) => unreachable(void),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(Right(err)),
            },
            Err(err) => Err(Left(err)),
        }
    }
}
