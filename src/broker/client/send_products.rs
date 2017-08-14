use futures::Future;
use hyper::StatusCode;
use serde_json::Value;

use common::json_response;
use common::messages::{Language, ProductName};
use super::{BoxedFuture, Client};

impl Client {
    /// Handles products being sent to the broker.
    pub fn send_products(self, name: ProductName, path: String, language: Option<Language>, product: Value) -> BoxedFuture {
        unimplemented!()
    }
}
