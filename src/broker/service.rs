//! The Service Protocol side of the Broker.

use std::collections::BTreeSet;

use futures::{Future, Poll};
use hyper::{Body, Client, Error as HyperError, StatusCode};
use hyper::client::HttpConnector;
use hyper::error::UriError;
use serde_json::Error as JsonError;
use tokio_core::reactor::Handle;

use broker::config::ServiceConfig;
use common::messages::ProtocolVersion;
use service::messages::{ServiceExtension, ServiceNegotiation};

/// A connection from the Broker to a Service.
pub struct Service {
    /// The configuration for connecting to the Service.
    pub config: ServiceConfig,

    /// The Service Protocol Extensions enabled.
    pub extensions: BTreeSet<ServiceExtension>,

    /// The ServiceNegotiation presented to the Broker.
    pub negotiation: ServiceNegotiation,

    /// The Service Protocol version being used to communicate to the Service.
    pub protocol: ProtocolVersion,

    client: Client<HttpConnector, Body>,
}

impl Service {
    /// Initiates a connection to the Service.
    pub fn connect(config: ServiceConfig, handle: &Handle) -> Box<Future<Item=Service, Error=ServiceConnectError>> {
        let client = Client::new(handle);
        let version_url = format!("{}://{}{}/version", config.scheme, config.addr, config.base).parse()
            .expect("TODO Proper error handling");
        Box::new(client.get(version_url).map_err(ServiceConnectError::from).map(|res| {
            unimplemented!()
        }))
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
