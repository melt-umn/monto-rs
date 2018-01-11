use futures::future::ok;
use hyper::{Response, StatusCode};
use serde_json::Value;

use monto3_common::json_response;
use monto3_protocol::{Language, Product, ProductName};
use monto3_protocol::client::BrokerPutError;

use client::{BoxedFuture, Client};

impl Client {
    /// Handles products being sent to the broker.
    pub fn send_products(
        self,
        name: ProductName,
        path: String,
        language: Option<Language>,
        value: Value,
    ) -> BoxedFuture {
        let language = match language
            .or_else(|| self.detect_language(&name, &path, &value))
        {
            Some(language) => language,
            None => {
                return json_response(
                    BrokerPutError::NoLanguage,
                    StatusCode::BadRequest,
                )
            }
        };

        /*
        let broker = self.0.borrow_mut();
        let mut cache = broker.cache.borrow_mut();

        let gp = Product {
            name,
            path,
            language,
            value,
        };
        cache.add(gp);

        Box::new(ok(Response::new().with_status(StatusCode::NoContent)))
        */
        unimplemented!()
    }

    /// Detects the language of a Product.
    ///
    /// TODO: Currently, not implemented (always returns `None`).
    /// Look into binding the `tokei` library, I guess?
    fn detect_language(
        &self,
        _name: &ProductName,
        _path: &str,
        _value: &Value,
    ) -> Option<Language> {
        None
    }
}
