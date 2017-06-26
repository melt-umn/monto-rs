//! The Client Protocol side of the Broker.

use broker::Broker;
use client::messages::ClientBrokerNegotiation;
use common::messages::ProtocolVersion;
use either::{Either, Left, Right};
use futures::future::{Empty, empty};
use futures::{Async, Future, Poll};
use hyper::Error as HyperError;
use hyper::server::{Http, Service};
use hyper::{Body, Request, Response};
use std::cell::RefCell;
use std::net::TcpListener;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio_core::net::TcpStream;
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

    /// Serves clients forever.
    pub fn run_forever(self) -> ! {
        let listener = TcpListener::bind(&self.config.net.addr)
            .expect("TODO proper error handling");
        let handle = self.handle.clone();
        let broker = Rc::new(RefCell::new(self));
        loop {
            let (stream, remote) = listener.accept()
                .expect("TODO proper error handling");
            let stream = TcpStream::from_stream(stream, &handle)
                .expect("TODO proper error handling");
            let service = Client(broker.clone());
            Http::new().bind_connection(&handle, stream, remote, service);
        }
    }
}

struct Client(Rc<RefCell<Broker>>);

impl Service for Client {
    type Request = Request;
    type Response = Response<Body>;
    type Error = HyperError;
    type Future = Box<Future<Item=Response<Body>, Error=HyperError>>;

    fn call(&self, req: Request) -> Self::Future {
        unimplemented!()
    }
}
