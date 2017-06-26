//! The Service Protocol side of the Broker.

use broker::config::ServiceConfig;
use common::messages::ProtocolVersion;
use futures::{Async, Future, Poll};
use hyper;
use service::messages::{ServiceExtension, ServiceNegotiation};
use std::collections::BTreeSet;
use tokio_core::reactor::Handle;

type HttpClient = hyper::client::Client<hyper::client::HttpConnector>;

/// A connection from the Broker to a Service.
pub struct Service {
    /// The Service Protocol Extensions enabled.
    pub extensions: BTreeSet<ServiceExtension>,

    /// The HTTP connection to the Service.
    pub http: HttpClient,

    /// The ServiceNegotiation presented to the Broker.
    pub negotiation: ServiceNegotiation,

    /// The Service Protocol version being used to communicate to the Service.
    pub protocol: ProtocolVersion,
}

impl Service {
    /// Initiates a connection to the Service.
    pub fn connect(config: &ServiceConfig, handle: &Handle) -> NewServiceFuture {
        debug!("Connecting to service: {:?}", config);
        unimplemented!()
    }
}

/// A Future for a single connection from the Broker to a Service.
pub struct NewServiceFuture(());

impl Future for NewServiceFuture {
    type Item = Service;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}
