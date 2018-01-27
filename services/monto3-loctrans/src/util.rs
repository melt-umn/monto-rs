use std::fmt::Display;

use monto3_common::messages::{Language, Product, ProductIdentifier,
    ProductName};
use monto3_service::messages::{ServiceError, ServiceNotice};
use serde_json::Value;

/// Extracts a product and the source for the given file.
pub fn product_and_src<E: Display, F: FnOnce(Value, String) -> Result<Value, E>>(
    path: &str,
    mut products: Vec<Product>,
    pn: ProductName,
    lang: Language,
    f: F,
) -> (Result<Value, Vec<ServiceError>>, Vec<ServiceNotice>) {
    let requested = if let Some(idx) = products
        .iter()
        .position(|p| p.name == pn && p.language == lang && p.path == path)
    {
        products.swap_remove(idx).value
    } else {
        return (
            Err(vec![
                ServiceError::UnmetDependency(ProductIdentifier {
                    name: pn,
                    language: lang,
                    path: path.to_string(),
                }),
            ]),
            vec![],
        );
    };
    let source = if let Some(idx) = products
        .iter()
        .position(|p| p.name == ProductName::Source && p.language == lang && p.path == path)
    {
        products.swap_remove(idx).value
    } else {
        return (
            Err(vec![
                ServiceError::UnmetDependency(ProductIdentifier {
                    name: ProductName::Source,
                    language: lang,
                    path: path.to_string(),
                }),
            ]),
            vec![],
        );
    };
    match source {
        Value::String(source) => {
            let r = f(requested, source).map_err(|e| {
                vec![ServiceError::Other(e.to_string())]
            });
            let n = products
                .into_iter()
                .map(|p| p.into())
                .map(ServiceNotice::UnusedDependency)
                .collect();
            (r, n)
        },
        _ => (
            Err(vec![
                ServiceError::Other("Invalid source product".to_string()),
            ]),
            vec![],
        ),
    }
}
