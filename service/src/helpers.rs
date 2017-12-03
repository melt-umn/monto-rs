//! Functions useful when defining a service.

use std::fmt::Display;

use serde_json::Value;

use monto3_common::messages::{Language, Product, ProductIdentifier, ProductName};

use messages::{ServiceError, ServiceNotice};

/// Serves as the body of a ServiceProvider that only operates on the source of
/// a single product.
pub fn one_to_one_fn<F: FnOnce(Value) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>)>(
    path: &str,
    mut products: Vec<Product>,
    pn: ProductName,
    lang: Language,
    f: F,
) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    let (r, mut n) = if let Some(idx) = products
        .iter()
        .position(|p| p.name == pn && p.language == lang && p.path == path)
    {
        f(products.swap_remove(idx).value)
    } else {
        (
            Err(vec![
                ServiceError::UnmetDependency(ProductIdentifier {
                    name: pn,
                    language: lang,
                    path: path.to_string(),
                }),
            ]),
            vec![],
        )
    };
    n.extend(
        products
            .into_iter()
            .map(|p| p.into())
            .map(ServiceNotice::UnusedDependency),
    );
    (r, n)
}

/// Serves as the body of a ServiceProvider that only operates on the source of
/// a single source product.
pub fn simple_fn<E: Display, F: FnOnce(String) -> Result<Value, E>>(
    path: &str,
    products: Vec<Product>,
    lang: Language,
    f: F,
) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    one_to_one_fn(path, products, ProductName::Source, lang, |val| match val {
        Value::String(s) => (
            f(s).map_err(|e| vec![ServiceError::Other(e.to_string())]),
            vec![],
        ),
        _ => (
            Err(vec![
                ServiceError::Other("Invalid source product".to_string()),
            ]),
            vec![],
        ),
    })
}
