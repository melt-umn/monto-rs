use std::cell::RefCell;
use std::io::Error as IoError;
use std::rc::Rc;

use either::{Either, Left, Right};
use futures::{Async, empty, Empty, Future, Poll, Stream};
use hyper::server::Http;
use tokio_core::net::{Incoming, TcpListener};
use tokio_core::reactor::Handle;
use void::Void;

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
                        unimplemented!()
                        // let service = Client(self.broker.clone());
                        // self.http.bind_connection(&self.handle, stream, remote, service);
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
