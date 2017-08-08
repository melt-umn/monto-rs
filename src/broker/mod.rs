//! An implementation of a Monto Broker.

pub mod client;
pub mod config;
pub mod service;

use futures::{Async, BoxFuture, Future, Poll};
use futures::future::{JoinAll, join_all};
use hyper::{Body, Client};
use tokio_core::reactor::Handle;

use self::config::Config;
use self::service::{Service, ServiceConnectError};

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
    pub fn new(config: Config, handle: Handle) -> Box<Future<Item=Broker, Error=ServiceConnectError>> {
        let futures = config.service.clone()
            .into_iter()
            .map(|s| Service::connect(&config, s, &handle))
            .collect::<Vec<_>>();
        Box::new(join_all(futures).map(|services| {
            Broker { config, handle, services }
        }))
    }
}
