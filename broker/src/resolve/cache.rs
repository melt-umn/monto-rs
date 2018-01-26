use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use serde_json::Value;

use monto3_protocol::{Product, ProductIdentifier};

/// A cache for products.
#[derive(Debug)]
pub struct Cache {
    /// A map from every product that has ever been in the cache to:
    ///
    ///  - The current version of the product (used for invalidation), and
    ///  - Possibly (unless the product has been invalidated (e.g. by being deleted on disk)):
    ///    - The value of the current version
    ///    - The version and identifier of each dependency.
    products: BTreeMap<
        ProductIdentifier,
        (Version, Option<(Value, Vec<(ProductIdentifier, Version)>)>),
    >,
}

impl Cache {
    /// Creates a new `Cache`.
    pub fn new() -> Cache {
        Cache {
            products: BTreeMap::new(),
        }
    }

    /// Inserts a new product into the cache, replacing any copies of the old
    /// one.
    pub fn add<I: IntoIterator<Item = ProductIdentifier>>(
        &mut self,
        product: Product,
        dependencies: I,
    ) {
        let identifier = ProductIdentifier {
            name: product.name,
            language: product.language,
            path: product.path,
        };
        let dependencies = dependencies
            .into_iter()
            .map(|id| (id.clone(), self.get_version(id)))
            .collect();
        match self.products.entry(identifier) {
            Entry::Vacant(entry) => {
                entry.insert(
                    (Version::default(), Some((product.value, dependencies))),
                );
            }
            Entry::Occupied(entry) => {
                let entry = entry.into_mut();
                entry.0.bump();
                entry.1 = Some((product.value, dependencies));
            }
        }
    }

    /// Gets the `Value` associated with a `ProductIdentifier`, if it exists
    /// and is valid.
    pub fn get(&self, identifier: ProductIdentifier) -> Option<&Value> {
        unimplemented!()
    }

    /// Gets the `Version` associated with a `ProductIdentifier` or creates an
    /// empty cache entry for the `ProductIdentifier`.
    fn get_version(&mut self, identifier: ProductIdentifier) -> Version {
        self.products
            .entry(identifier)
            .or_insert((Version::default(), None))
            .0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Version(usize);

impl Version {
    fn bump(&mut self) {
        *self = self.next();
    }

    fn next(self) -> Version {
        let Version(n) = self;
        Version(n + 1)
    }
}
