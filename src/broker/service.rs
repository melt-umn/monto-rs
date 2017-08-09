//! The Service Protocol side of the Broker.

use std::cmp::min;
use std::collections::BTreeSet;

use futures::{Future, Poll, Stream};
use futures::future::{err, ok, result};
use hyper::{Body, Chunk, Client, Error as HyperError, Method, Request, StatusCode};
use hyper::client::HttpConnector;
use hyper::error::UriError;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error as JsonError;
use tokio_core::reactor::Handle;

use broker::config::{Config, ServiceConfig};
use common::messages::ProtocolVersion;
use service::messages::{ServiceExtension, ServiceBrokerNegotiation, ServiceNegotiation};

/// A connection from the Broker to a Service.
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

    client: Client<HttpConnector, Body>,
}

impl Service {
    /// Initiates a connection to the Service.
    pub fn connect(config: Config, service_config: ServiceConfig, handle: &Handle) -> Box<Future<Item=Service, Error=ServiceConnectError>> {
        let client = Client::new(handle);
        let version_uri = format!("{}://{}{}/version", service_config.scheme,
                service_config.addr, service_config.base)
            .parse()
            .expect("TODO Proper error handling");
        let mut request = Request::new(Method::Post, version_uri);
        let our_version = ProtocolVersion {
            major: 3,
            minor: 0,
            patch: 0,
        };
        let sbn = ServiceBrokerNegotiation {
            monto: our_version,
            broker: config.version.clone().into(),
            extensions: config.extensions.service.clone(),
        };
        match serde_json::to_string(&sbn) {
            Ok(sbn) => request.set_body(sbn),
            Err(e) => return Box::new(err(e.into())),
        }
        Box::new(client.request(request).map_err(ServiceConnectError::from).and_then(|res| {
            match res.status() {
                StatusCode::Ok => {
                    res.body()
                        .concat2()
                        .map_err(ServiceConnectError::from)
                },
                _ => panic!("TODO Error handling"),
            }
        }).and_then(|body: Chunk| {
            result(serde_json::from_slice(body.as_ref()))
                .map_err(ServiceConnectError::from)
        }).and_then(move |sn: ServiceNegotiation| {
            let version = min(our_version, sn.monto);
            let extensions = config.extensions.service.intersection(&sn.extensions)
                .cloned()
                .collect();
            ok(Service {
                client,
                config: service_config,
                extensions,
                negotiation: sn,
                protocol: version,
            })
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
