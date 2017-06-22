//! Functions and types useful for implementing the Client Protocol, as defined
//! in
//! [Section 4](https://melt-umn.github.io/monto-v3-draft/draft02/#4-the-client-protocol)
//! of the specification.

pub mod messages;

use common::messages::{Identifier, Product, ProductIdentifier};
use self::messages::{BrokerGetError, BrokerPutError};
use std::collections::{BTreeMap, BTreeSet};

/// A Monto Client.
pub struct Client {
    services: BTreeMap<Identifier, BTreeSet<ProductIdentifier>>,
}

impl Client {
    /// Attempts to retrieve a Product from the Broker, as described in
    /// [Section 4.4](https://melt-umn.github.io/monto-v3-draft/draft02/#4-4-requesting-products)
    /// of the specification.
    pub fn request(&mut self, service: &Identifier, product: &ProductIdentifier) -> Result<Product, BrokerGetError> {
        unimplemented!()
    }

    /// Returns an iterator over the Products that can be requested by the Client.
    pub fn products(&self) -> ClientProductIter {
        unimplemented!()
    }

    /// Sends a Product to the Broker, as described in
    /// [Section 4.3](https://melt-umn.github.io/monto-v3-draft/draft02/#4-3-sending-products)
    /// of the specification.
    pub fn send(&mut self, p: Product) -> Result<(), BrokerPutError> {
        unimplemented!()
    }
}

/// An iterator over the products a Client can request.
pub struct ClientProductIter<'a>(&'a ());

impl<'a> Iterator for ClientProductIter<'a> {
    type Item = (&'a Identifier, &'a ProductIdentifier);
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
