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
        Box::new(self.resolve(service_id, product, vec![]).then(|r| match r {
            Ok(product) => json_response(product, StatusCode::Ok),
            Err(err) => {
                let status = match err {
                    BrokerGetError::NoSuchService => StatusCode::BadRequest,
                    BrokerGetError::NoSuchProduct => StatusCode::BadRequest,
                    BrokerGetError::ServiceError { .. } => StatusCode::InternalServerError,
                    BrokerGetError::ServiceConnectError { .. } => StatusCode::BadGateway,
                    BrokerGetError::Unresolvable(_) => StatusCode::InternalServerError,
                };
                json_response(err, status)
            },
        }))
    }
}
