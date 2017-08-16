//! Dependency resolution and product caching for the broker.

mod cache;
mod watcher;

use futures::Future;

use client::messages::BrokerGetError;
use common::messages::{GenericProduct, Identifier, ProductDescriptor, ProductIdentifier};
pub use self::cache::Cache;
use super::{Broker, Service};

impl Broker {
    /// Fully resolves a product request, including doing dependency resolution.
    pub fn resolve(&mut self, si: Identifier, pi: ProductIdentifier) -> Box<Future<Item=GenericProduct, Error=BrokerGetError>> {
        unimplemented!()
    }

    /// Tries to retrieve a product from the cache.
    fn from_cache(&self, pi: ProductIdentifier) -> Option<GenericProduct> {
        let cache = self.cache.borrow();
        cache.get(pi)
    }

    /// Finds a service that can respond to the request for the given product.
    fn service_for(&self, pd: &ProductDescriptor) -> Option<Identifier> {
        self.services.iter()
            .find(|s| s.negotiation.products.contains(pd))
            .map(|s| s.negotiation.service.id.clone())
    }
}
