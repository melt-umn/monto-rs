//! An implementation of a Monto Broker.

pub mod client;
pub mod config;
pub mod service;

use futures::{Async, Future, Poll};
use futures::future::{JoinAll, join_all};
use tokio_core::reactor::Handle;

use self::config::Config;
use self::service::{NewServiceFuture, Service};

/// The Broker.
pub struct Broker {
    config: Config,
    handle: Handle,
    services: Vec<Service>,

    // TODO
}

impl Broker {
    /// Creates a new instance of the Broker, returning a Future for the
    /// constructed Broker.
    ///
    /// TODO: This can be made more efficient when
    /// [`conservative_impl_trait`](https://github.com/rust-lang/rust/issues/34511)
    /// is stabilized.
    pub fn new(config: Config, handle: Handle) -> NewFuture {
        let futures = config.service.iter()
            .map(|s| Service::connect(s, &handle))
            .collect();
        NewFuture(join_all(futures), Some(config), Some(handle))
    }
}

/// A Future for the Broker connecting to Services and starting to listen for
/// Clients.
pub struct NewFuture(JoinAll<Vec<NewServiceFuture>>, Option<Config>, Option<Handle>);

impl Future for NewFuture {
    type Item = Broker;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0.poll() {
            Ok(Async::Ready(services)) => Ok(Async::Ready(Broker {
                config: self.1.take().unwrap(),
                handle: self.2.take().unwrap(),
                services,
            })),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err),
        }
    }
}
