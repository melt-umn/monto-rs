//! An implementation of a Monto Broker.

#[macro_use]
extern crate clap;
extern crate dirs;
extern crate either;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mime;
extern crate monto3_common;
extern crate monto3_protocol;
extern crate notify;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate toml;
extern crate url;
extern crate void;

pub mod client;
pub mod config;
mod consts;
mod errors;
pub mod resolve;
pub mod service;

use std::cell::RefCell;
use std::rc::Rc;

use futures::Future;
use futures::future::{err, join_all};
use notify::Error as NotifyError;
use tokio_core::reactor::Handle;

use monto3_protocol::client::ClientBrokerNegotiation;
use monto3_protocol::{Identifier, ProtocolVersion, SoftwareVersion};
use monto3_protocol::service::ServiceBrokerNegotiation;

use config::Config;
pub use errors::{Error, ErrorKind, Result, ResultExt};
use resolve::{Cache, Watcher};
use service::Service;

/// The Broker.
pub struct Broker {
    cache: Cache,
    config: Config,
    handle: Handle,
    services: Vec<Service>,
    watcher: Watcher,
}

impl Broker {
    /// Creates a new instance of the Broker, returning a Future for the
    /// constructed Broker.
    ///
    /// TODO: This can be made more efficient when
    /// [`conservative_impl_trait`](https://github.com/rust-lang/rust/issues/34511)
    /// is stabilized.
    pub fn new(
        config: Config,
        handle: Handle,
    ) -> Box<Future<Item = Broker, Error = Error>> {
        let watcher = match Watcher::new() {
            Ok(watcher) => watcher,
            Err(e) => return Box::new(err(e.into())),
        };
        let futures = config
            .service
            .clone()
            .into_iter()
            .map(|s| {
                Service::connect(config.clone(), s, handle.clone())
                    .map_err(Error::from)
            })
            .collect::<Vec<_>>();
        Box::new(join_all(futures).map(|services| {
            info!("Connected to all services: {:?}", services);
            Broker {
                cache: Cache::new(),
                config,
                handle,
                services,
                watcher,
            }
        }))
    }

    /// Returns the service with the given id, if one exists.
    pub fn find_service(&self, id: &Identifier) -> Option<&Service> {
        for service in self.services.iter() {
            if &service.negotiation.service.id == id {
                return Some(service);
            }
        }
        None
    }

    /// Creates a ClientBrokerNegotiation.
    pub fn client_negotiation(&self) -> ClientBrokerNegotiation {
        ClientBrokerNegotiation {
            monto: ProtocolVersion {
                major: 3,
                minor: 0,
                patch: 0,
            },
            broker: self.version(),
            extensions: self.config.extensions.client.clone(),
            services: self.services
                .iter()
                .map(|s| s.negotiation.clone())
                .collect(),
        }
    }

    /// Creates a ServiceBrokerNegotiation.
    pub fn service_negotiation(&self) -> ServiceBrokerNegotiation {
        ServiceBrokerNegotiation {
            monto: ProtocolVersion {
                major: 3,
                minor: 0,
                patch: 0,
            },
            extensions: self.config.extensions.service.clone(),
            broker: self.version(),
        }
    }

    /// Returns the version information for the Broker.
    pub fn version(&self) -> SoftwareVersion {
        self.config.version.clone().into()
    }
}
