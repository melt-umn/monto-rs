//! The Service Protocol side of the Broker.

mod connect;
mod request;

use std::collections::BTreeSet;

use hyper::Body;
use hyper::client::{Client, HttpConnector};
use tokio_core::reactor::Handle;

use monto3_protocol::ProtocolVersion;
use monto3_protocol::service::{ServiceExtension, ServiceNegotiation};

use config::{Config, ServiceConfig};

pub use self::connect::{ServiceConnectError, ServiceConnectErrorKind,
                        ServiceConnectFuture};
pub use self::request::{ServiceRequestError, ServiceRequestErrorKind,
                        ServiceRequestFuture};

/// A connection from a Broker to a Service.
#[derive(Debug)]
pub struct Service {
    /// The configuration for connecting to the Service.
    pub config: ServiceConfig,

    /// The Service Protocol Extensions enabled.
    pub extensions: BTreeSet<ServiceExtension>,

    /// The ServiceNegotiation presented to the Broker.
    pub negotiation: ServiceNegotiation,

    /// The Service Protocol version being used to communicate to the Service.
    pub protocol: ProtocolVersion,

    http: Client<HttpConnector, Body>,
}

impl Service {
    pub fn connect(
        config: Config,
        service_config: ServiceConfig,
        handle: &Handle,
    ) -> ServiceConnectFuture {
        unimplemented!()
    }
}
