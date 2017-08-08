use std::cell::RefCell;
use std::io::Error as IoError;
use std::rc::Rc;

use either::{Either, Left, Right};
use futures::{Async, empty, Empty, Future, Poll, Stream};
use futures::future::err;
use hyper::{Body, Error as HyperError, Method, Request, Response, StatusCode};
use hyper::server::{Http, Service as HyperService};
use log::LogLevel;
use tokio_core::net::{Incoming, TcpListener};
use tokio_core::reactor::Handle;
use void::Void;

use common::{error_response, json_request, json_response};
use service::messages::ServiceBrokerNegotiation;
use super::Service;

impl Service {
    /// Serves until the given future resolves.
    pub fn serve_until<F: Future>(self, stop: F) -> ServeFuture<F> {
        let listener = TcpListener::bind(&self.config.net.addr, &self.handle)
            .expect("TODO proper error handling")
            .incoming();
        let handle = self.handle.clone();
        let service = Rc::new(RefCell::new(self));
        ServeFuture {
            handle,
            http: Http::new(),
            listener,
            service,
            stop,
        }
    }

    /// Serves forever.
    pub fn serve_forever(self) -> ServeFuture<Empty<Void, Void>> {
        self.serve_until(empty())
    }
}

struct Broker(Rc<RefCell<Service>>);

impl HyperService for Broker {
    type Request = Request;
    type Response = Response<Body>;
    type Error = HyperError;
    type Future = Box<Future<Item=Response<Body>, Error=HyperError>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, _, body) = req.deconstruct();
        Box::new(match (method.clone(), uri.path()) {
            (Method::Post, "/monto/version") => {
                let service = self.0.clone();
                Box::new(json_request(body).and_then(move |sbn: ServiceBrokerNegotiation| {
                    debug!("Got ServiceBrokerNegotiation {:?}", sbn);
                    let sn = service.borrow().negotiation();
                    let status = if sbn.monto.compatible(&sn.monto) {
                        StatusCode::Ok
                    } else {
                        StatusCode::BadRequest
                    };
                    json_response(sn, status)
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

/// A Future for a Service serving to Brokers.
pub struct ServeFuture<F: Future> {
    handle: Handle,
    http: Http,
    listener: Incoming,
    service: Rc<RefCell<Service>>,
    stop: F,
}

impl<F: Future> Future for ServeFuture<F> {
    type Item = F::Item;
    type Error = Either<F::Error, IoError>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.stop.poll() {
            Ok(Async::NotReady) => loop {
                match self.listener.poll() {
                    Ok(Async::Ready(Some((stream, remote)))) => {
                        info!("Got connection from {}", remote);
                        let service = Broker(self.service.clone());
                        self.http.bind_connection(&self.handle, stream, remote, service);
                    },
                    Ok(Async::Ready(None)) => {
                        panic!("TcpListener.incoming() stream ended! (This is documented to be impossible)");
                    },
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(err) => return Err(Right(err)),
                }
            },
            Ok(Async::Ready(x)) => Ok(Async::Ready(x)),
            Err(e) => Err(Left(e)),
        }
    }
}
