//! Functions useful when defining a service.

use std::fmt::Display;

use Value;
use common::messages::{Language, Product, ProductIdentifier, ProductName};
use service::messages::{ServiceError, ServiceNotice};

/// Serves as the body of a ServiceProvider that only operates on the source of
/// a single product.
pub fn one_to_one_fn<E: Display, F: FnOnce(String) -> Result<Value, E>>(
    path: &str,
    mut products: Vec<Product>,
    pn: ProductName,
    lang: Language,
    f: F,
) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    let idx = products.iter().position(|p| {
        p.name == pn && p.language == lang && p.path == path
    });

    let r = if let Some(idx) = idx {
        match products.swap_remove(idx).value {
            Value::String(src) => f(src).map_err(|e| ServiceError::Other(e.to_string())),
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

/// Serves as the body of a ServiceProvider that only operates on the source of
/// a single source product.
pub fn simple_fn<E: Display, F: FnOnce(String) -> Result<Value, E>>(
    path: &str,
    products: Vec<Product>,
    lang: Language,
    f: F,
) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    one_to_one_fn(path, products, ProductName::Source, lang, f)
}
