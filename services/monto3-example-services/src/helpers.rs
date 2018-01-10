use std::fmt::Display;

use monto3_common::messages::{Language, Product, ProductIdentifier,
                              ProductName};
use monto3_service::messages::{ServiceError, ServiceNotice};
use serde_json::Value;

pub fn simple_fn<E: Display, F: FnOnce(String) -> Result<Value, E>>(
    path: &str,
    mut products: Vec<Product>,
    lang: Language,
    f: F,
) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    let idx = products.iter().position(|p| {
        p.name == ProductName::Source && p.language == lang && p.path == path
    });

    let r = if let Some(idx) = idx {
        match products.swap_remove(idx).value {
            Value::String(src) => {
                f(src).map_err(|e| ServiceError::Other(e.to_string()))
            }
            _ => Err(ServiceError::Other("bad source product".to_string())),
        }
    } else {
        Err(ServiceError::UnmetDependency(ProductIdentifier {
            name: ProductName::Source,
            language: lang,
            path: path.to_string(),
        }))
    };
    let notices = products
        .into_iter()
        .map(|p| p.into())
        .map(ServiceNotice::UnusedDependency)
        .collect();
    (
        match r {
            Ok(product) => Ok(product),
            Err(err) => Err(vec![err]),
        },
        notices,
    )
}
