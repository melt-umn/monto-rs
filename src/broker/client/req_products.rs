use either::{Left, Right};
use futures::Future;
use hyper::StatusCode;

use broker::service::RequestError;
use common::json_response;
use common::messages::{GenericProduct, Identifier, ProductIdentifier};
use client::messages::{BrokerGetError, ClientNegotiation};
use super::{BoxedFuture, Client};

impl Client {
    /// Handles a request for products sent to the broker.
    pub fn req_products(self, service_id: Identifier, product: ProductIdentifier) -> BoxedFuture {
        let broker = self.0.borrow();
        if let Some(service) = broker.find_service(&service_id) {
            Box::new(service.request(&product).then(move |r: Result<GenericProduct, RequestError>| {
                match r {
                    Ok(p) => json_response(p, StatusCode::Ok),
                    Err(err) => match err.kind() {
                        _ => json_response(BrokerGetError::ServiceConnectError {
                            service: service_id,
                            error: format!("{}", err),
                        }, StatusCode::BadGateway),
                    },
                }
            }))
        } else {
            json_response(BrokerGetError::NoSuchService, StatusCode::BadRequest)
        }
    }
}
