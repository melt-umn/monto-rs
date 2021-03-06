//! Functions and types useful for implementing the Service Protocol, as defined
//! in
//! [Section 5](https://melt-umn.github.io/monto-v3-draft/draft03/#5-the-service-protocol)
//! of the specification.
//!
//! This ought to be rewritten to use a trait for ServiceFn.

#[macro_use]
extern crate clap;
extern crate dirs;
extern crate either;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
#[doc(hidden)]
pub extern crate monto3_common;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[doc(hidden)]
pub extern crate serde_json;
extern crate tokio_core;
extern crate toml;
extern crate void;

#[macro_use]
mod macros;

pub mod config;
pub mod helpers;
pub mod messages;
mod serve;

use std::collections::BTreeMap;

use serde_json::Value;
use tokio_core::reactor::Handle;

use monto3_common::messages::{Product, ProductDescriptor, ProtocolVersion};

use config::Config;
use messages::{ServiceError, ServiceNegotiation, ServiceNotice};
pub use serve::ServeFuture;

/// A Service and the associated HTTP server.
pub struct Service {
    config: Config,
    funcs: BTreeMap<ProductDescriptor, Box<ServiceProvider>>,
    handle: Handle,
}

impl Service {
    /// Creates a new Service.
    pub fn new(config: Config, handle: Handle) -> Service {
        let funcs = BTreeMap::new();
        Service {
            config,
            funcs,
            handle,
        }
    }

    /// Creates a ServiceNegotiation.
    pub fn negotiation(&self) -> ServiceNegotiation {
        ServiceNegotiation {
            extensions: self.config.extensions.clone(),
            monto: ProtocolVersion {
                major: 3,
                minor: 0,
                patch: 0,
            },
            products: self.funcs.keys().cloned().collect(),
            service: self.config.version.clone().into(),
        }
    }

    /// Adds a ServiceProvider to the service.
    ///
    /// Replaces any ServiceProvider that provides the same Product.
    pub fn add_provider<P: ServiceProvider + 'static>(&mut self, provider: P) {
        let descriptor = provider.descriptor();
        self.funcs.insert(descriptor, Box::new(provider));
    }
}

/// A function for a service.
pub trait ServiceProvider {
    /// Returns a ProductDescriptor for the product this provider provides.
    fn descriptor(&self) -> ProductDescriptor;

    /// The function that actually runs the service.
    fn service(
        &mut self,
        path: &str,
        products: Vec<Product>,
    ) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>);
}
