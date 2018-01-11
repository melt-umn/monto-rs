use futures::{Async, Future};
use hyper::{Body, Chunk, Client, Error as HyperError, Method, Request,
            StatusCode};
use hyper::error::UriError;
use itertools::Itertools;
use serde_json::Error as JsonError;

use monto3_protocol::{Product, ProductDescriptor, ProductIdentifier,
                      ProtocolVersion};
use monto3_protocol::service::ServiceErrors;

use service::Service;

/// A future for requesting a product from a `Service`.
pub struct ServiceRequestFuture<'service> {
    service: &'service Service,
}

impl<'service> Future for ServiceRequestFuture<'service> {
    type Item = Service;
    type Error = ServiceRequestError;
    fn poll(&mut self) -> Result<Async<Service>, ServiceRequestError> {
        unimplemented!()
    }
}

error_chain! {
    types {
        ServiceRequestError, ServiceRequestErrorKind, ServiceRequestResultExt;
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
        /// The given product is not exposed by the service.
        NotExposed(desc: ProductDescriptor) {
            description("The given product is not exposed by the service")
            display("The product {} for language {} is not exposed by the service", desc.name, desc.language)
        }
        /// Errors sent from the service.
        ServiceErrors(errors: ServiceErrors) {
            description("Errors sent from the service")
            display("Errors sent from the service: {:?}", errors.errors.iter().format(", "))
        }
    }
}
