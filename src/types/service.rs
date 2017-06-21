use std::collections::BTreeSet;
use super::broker::BrokerVersion;
use super::common::{Identifier, MontoVersion, NamespacedName, Product, ProductIdentifier};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ServiceBrokerNegotiation {
    pub monto: MontoVersion,
    pub broker: BrokerVersion,
    #[serde(default)]
    pub extensions: BTreeSet<ServiceExtension>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ServiceNegotiation {
    pub monto: MontoVersion,
    pub service: ServiceVersion,
    #[serde(default)]
    pub extensions: BTreeSet<ServiceExtension>,
    pub products: BTreeSet<ProductIdentifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ServiceExtension {
    Unknown(NamespacedName),
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ServiceVersion {
    pub id: Identifier,
    pub name: Option<String>,
    pub vendor: Option<String>,
    #[serde(default)]
    pub major: u64,
    #[serde(default)]
    pub minor: u64,
    #[serde(default)]
    pub patch: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BrokerRequest {
    #[serde(default)]
    pub products: Vec<Product>,
    pub request: ProductIdentifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServiceErrors {
    pub errors: Vec<ServiceError>,
    pub notices: Vec<ServiceNotice>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ServiceError {
    UnmetDependency(ProductIdentifier),
    Other(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServiceProduct {
    pub product: Product,
    pub notices: Vec<ServiceNotice>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ServiceNotice {
    UnusedDependency(ProductIdentifier),
}
