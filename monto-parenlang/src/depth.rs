use itertools::partition;

use monto::common::messages::{GenericProduct, Language, Product, ProductDescriptor, ProductIdentifier, ProductName};
use monto::service::ServiceProvider;
use monto::service::messages::{ServiceError, ServiceErrors, ServiceNotice, ServiceProduct};

use parenlang::Ast;

/// Measures the maximum depth of parentheses.
pub struct DepthProvider;

impl ServiceProvider for DepthProvider {
    fn descriptor(&self) -> ProductDescriptor {
        ProductDescriptor {
            name: "edu.umn.cs.melt.monto_rs.balanced_parens.depth".parse().unwrap(),
            language: Language::Other("balanced-parens".to_string()),
        }
    }

    fn service(&mut self, path: &str, mut products: Vec<GenericProduct>) -> Result<ServiceProduct<GenericProduct>, ServiceErrors> {
        let language = Language::Other("balanced-parens".to_string());
        let idx = products.iter().position(|p| {
            !(p.name() == ProductName::Source && p.language() == language && p.path() == path)
        });

        let r = if let Some(idx) = idx {
            let src = products.swap_remove(idx);
            Ok(unimplemented!())
        } else {
            Err(ServiceError::UnmetDependency(ProductIdentifier {
                name: ProductName::Source,
                language: Language::Other("balanced-parens".to_string()),
                path: path.to_string(),
            }))
        };
        let notices = products.into_iter()
            .map(|p| p.identifier())
            .map(ServiceNotice::UnusedDependency)
            .collect();
        match r {
            Ok(product) => Ok(ServiceProduct {
                product,
                notices,
            }),
            Err(err) => Err(ServiceErrors {
                errors: vec![err],
                notices,
            }),
        }
    }
}

fn depth(a: &Ast) -> usize {
    match a.0.iter().map(depth).max() {
        Some(x) => 1 + x,
        None => 0,
    }
}
