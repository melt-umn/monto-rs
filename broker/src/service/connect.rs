use std::cmp::min;
use std::collections::BTreeSet;

use futures::{Async, Future, Stream};
use futures::future::{err, ok, result};
use hyper::{Body, Chunk, Client, Error as HyperError, Method, Request,
            StatusCode};
use hyper::client::HttpConnector;
use hyper::error::UriError;
use hyper::header::ContentType;
use itertools::Itertools;
use serde_json;
use serde_json::Error as JsonError;
use tokio_core::reactor::Handle;

use monto3_protocol::{Product, ProductDescriptor, ProductIdentifier,
                      ProtocolVersion};
use monto3_protocol::service::{BrokerRequest, ServiceBrokerNegotiation,
                               ServiceErrors, ServiceExtension,
                               ServiceNegotiation, ServiceProduct};

use config::{Config, ServiceConfig};
use service::Service;

/// A future for connecting to a `Service`.
pub struct ServiceConnectFuture<'handle> {
    config: Config,
    service_config: ServiceConfig,
    handle: &'handle Handle,
}

impl<'handle> Future for ServiceConnectFuture<'handle> {
    type Item = Service;
    type Error = ServiceConnectError;
    fn poll(&mut self) -> Result<Async<Service>, ServiceConnectError> {
        unimplemented!()
    }
}

error_chain! {
    types {
        ServiceConnectError, ServiceConnectErrorKind, ServiceConnectResultExt;
    }
    foreign_links {
        Hyper(HyperError)
            #[doc = "An error from the network."];
        Serde(JsonError)
            #[doc = "An invalid response was received."];
        Uri(UriError)
            #[doc = "An invalid URI was created from the config"];
    }
    errors {
        /// A status other than Ok was received from the Broker, indicating
        /// that the Client is not compatible.
        BadStatus(code: StatusCode) {
            description("The Broker is not compatible with this Client")
            display("The Broker is not compatible with this Client: got {} from the Broker", code)
        }

        /// The Broker and Service are not compatible.
        NotCompatible(broker: ProtocolVersion, service: ProtocolVersion) {
            description("The Broker and Service are not compatible")
            display("The Broker (Monto version {}) and Service (Monto version {}) are not compatible.", broker, service)
        }
    }
}
