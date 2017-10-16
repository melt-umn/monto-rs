//! An implementation of a Monto Broker.

pub mod client;
pub mod config;
pub mod resolve;
pub mod service;

use std::cell::RefCell;
use std::rc::Rc;

use futures::Future;
use futures::future::{err, join_all};
use notify::Error as NotifyError;
use tokio_core::reactor::Handle;

use client::messages::ClientBrokerNegotiation;
use common::messages::{Identifier, ProtocolVersion, SoftwareVersion};
use self::config::Config;
use self::resolve::Cache;
use self::service::{Service, ServiceConnectError, ServiceConnectErrorKind};
use service::messages::ServiceBrokerNegotiation;

/// The Broker.
pub struct Broker {
    cache: Rc<RefCell<Cache>>,
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
    pub fn new(
        config: Config,
        handle: Handle,
    ) -> Box<Future<Item = Broker, Error = NewBrokerError>> {
        let cache = match Cache::new(&handle) {
            Ok(cache) => cache,
            Err(e) => return Box::new(err(e.into())),
        };
        let futures = config
            .service
            .clone()
            .into_iter()
            .map(|s| {
                Service::connect(config.clone(), s, &handle).map_err(NewBrokerError::from)
            })
            .collect::<Vec<_>>();
        Box::new(join_all(futures).map(|services| {
            info!("Connected to all services: {:?}", services);
            Broker {
                cache,
                config,
                handle,
                services,
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

error_chain! {
    types {
        NewBrokerError, NewBrokerErrorKind, NewBrokerResultExt;
    }
    foreign_links {
        Notify(NotifyError)
            #[doc = "An error setting up the notifier."];
    }
    links {
        ServiceConnect(ServiceConnectError, ServiceConnectErrorKind)
            #[doc = "An error connecting to a service."];
    }
}
