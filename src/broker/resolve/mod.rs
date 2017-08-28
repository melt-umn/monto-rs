//! Dependency resolution and product caching for the broker.

mod cache;
mod watcher;

use futures::Future;
use futures::future::{err, ok};

use broker::client::Client;
use broker::service::{RequestError, RequestErrorKind};
use client::messages::BrokerGetError;
use common::messages::{Identifier, Product, ProductDescriptor, ProductIdentifier};
pub use self::cache::Cache;
use service::messages::{ServiceError, ServiceErrors, ServiceNotice};
use super::Broker;

impl Client {
    /// Fully resolves a product request, including doing dependency resolution.
    pub fn resolve(self, si: Identifier, pi: ProductIdentifier, mut ps: Vec<Product>) -> Box<Future<Item=Product, Error=BrokerGetError>> {
        let self2 = self.clone();
        let broker = self2.0.borrow();
        info!("getting {:?} from {}", pi, si);

        if let Some(gp) = broker.from_cache(pi.clone()) {
            Box::new(ok(gp))
        } else {
            if let Some(service) = broker.find_service(&si) {
                Box::new(service.request(pi.clone(), &ps).then(move |r| match r {
                    Ok(sp) => Box::new(ok(sp.product)),
                    Err(RequestError(e, _)) => {
                        error!("{}", e);
                        match e {
                            RequestErrorKind::Hyper(e) => {
                                Box::new(err(BrokerGetError::ServiceConnectError {
                                    service: si,
                                    error: e.to_string(),
                                }))
                            },
                            RequestErrorKind::ServiceErrors(ServiceErrors {
                                errors,
                                notices,
                            }) => {
                                for ServiceNotice::UnusedDependency(pi) in notices {
                                    let idx = ps.iter()
                                        .cloned()
                                        .map(ProductIdentifier::from)
                                        .position(|pi2| pi2 == pi);
                                    if let Some(idx) = idx {
                                        ps.swap_remove(idx);
                                    } else {
                                        warn!("Couldn't find {:?} in {:?}", pi, ps);
                                    }
                                }
                                self.resolve_next(si, pi, ps, errors)
                            },
                            _ => {
                                Box::new(err(BrokerGetError::ServiceError {
                                    service: si,
                                    error: e.to_string(),
                                }))
                            },
                        }
                    },
                }))
            } else {
                Box::new(err(BrokerGetError::NoSuchService))
            }
        }
    }

    /// Resolves from any service.
    fn resolve_dep(self, pi: ProductIdentifier) -> Box<Future<Item=Product, Error=BrokerGetError>> {
        let service = {
            let broker = self.0.borrow();
            if let Some(gp) = broker.from_cache(pi.clone()) {
                return Box::new(ok(gp));
            } else {
                let pd = ProductDescriptor {
                    name: pi.name.clone(),
                    language: pi.language.clone(),
                };
                broker.services.iter()
                    .find(|s| s.negotiation.products.contains(&pd))
                    .map(|s| s.negotiation.service.id.clone())
            }
        };
        if let Some(si) = service {
            self.resolve(si, pi, vec![])
        } else {
            Box::new(err(BrokerGetError::Unresolvable(pi)))
        }
    }

    /// Handles the error case of resolve.
    fn resolve_next(self, si: Identifier, pi: ProductIdentifier, mut ps: Vec<Product>, mut es: Vec<ServiceError>) -> Box<Future<Item=Product, Error=BrokerGetError>> {
        if let Some(se) = es.pop() {
            match se {
                ServiceError::UnmetDependency(pi2) => {
                    Box::new(self.clone().resolve_dep(pi2).and_then(|p| {
                        ps.push(p);
                        self.resolve_next(si, pi, ps, es)
                    }))
                },
                ServiceError::Other(s) => {
                    Box::new(err(BrokerGetError::ServiceError {
                        service: si,
                        error: s,
                    }))
                },
            }
        } else {
            self.resolve(si, pi, ps)
        }
    }
}

impl Broker {
    /// Tries to retrieve a product from the cache.
    fn from_cache(&self, pi: ProductIdentifier) -> Option<Product> {
        let cache = self.cache.borrow();
        cache.get(pi)
    }
}
