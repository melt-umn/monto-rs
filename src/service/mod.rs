//! Functions and types useful for implementing the Service Protocol, as defined
//! in
//! [Section 5](https://melt-umn.github.io/monto-v3-draft/draft02/#5-the-service-protocol)
//! of the specification.
//!
//! This ought to be rewritten to use a trait for ServiceFn.

pub mod config;
pub mod messages;

use std::collections::BTreeMap;

use common::messages::{Product, ProductDescriptor, ProductName};
use self::config::Config;
use self::messages::{ServiceErrors, ServiceNegotiation, ServiceNotice};

/// A function for a service.
pub type ServiceFn = Fn() -> (Result<Box<Product>, ServiceErrors>, Vec<ServiceNotice>);

/// A Service and the associated HTTP server.
pub struct Service {
    config: Config,
    funcs: BTreeMap<ProductName, Box<ServiceFn>>,
}

impl Service {
    /// Creates a new Service.
    pub fn new(config: Config) -> Service {
        unimplemented!()
    }

    /// Creates a ServiceNegotiation.
    pub fn negotiation(&self) -> ServiceNegotiation {
        ServiceNegotiation {
            extensions: self.config.extensions.clone(),
            monto: unimplemented!(),
            products: unimplemented!(),
            service: unimplemented!(),
        }
    }
}
