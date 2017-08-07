use itertools::partition;

use monto::common::messages::{Language, Product, ProductDescriptor, ProductName};
use monto::service::ServiceProvider;
use monto::service::messages::{ServiceErrors, ServiceNotice};

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

    fn service(&mut self, path: &str, mut products: Vec<Box<Product>>) -> (Result<Box<Product>, ServiceErrors>, Vec<ServiceNotice>) {
        let language = Language::Other("balanced-parens".to_string());
        let idx = products.iter().position(|p| {
            !(p.name() == ProductName::Source && p.language() == language && p.path() == path)
        });

        let r = if let Some(idx) = idx {
            let src = products.swap_remove(idx);
            Ok(unimplemented!())
        } else {
            unimplemented!()
        };
        let notices = products.into_iter()
            .map(|p| p.identifier())
            .map(ServiceNotice::UnusedDependency)
            .collect();

        (r, notices)
    }
}

fn depth(a: &Ast) -> usize {
    match a.0.iter().map(depth).max() {
        Some(x) => 1 + x,
        None => 0,
    }
}
