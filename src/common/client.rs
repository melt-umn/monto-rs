//! Types useful for implementing a Monto Client.

use std::collections::BTreeSet;
use super::{Identifier, MontoVersion, NamespacedName, Product, ProductIdentifier};
use super::broker::BrokerVersion;
use super::service::{ServiceError, ServiceNegotiation};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClientNegotiation {
    pub monto: MontoVersion,
    pub client: ClientVersion,
    #[serde(default)]
    pub extensions: BTreeSet<ClientExtension>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClientVersion {
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

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClientBrokerNegotiation {
    pub monto: MontoVersion,
    pub broker: BrokerVersion,
    #[serde(default)]
    pub extensions: BTreeSet<ClientExtension>,
    pub services: Vec<ServiceNegotiation>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ClientExtension {
    Unknown(NamespacedName),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClientRequest {
    #[serde(default)]
    pub products: Vec<Product>,
    pub requests: Vec<ClientSingleRequest>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClientSingleRequest {
    pub product: ProductIdentifier,
    pub service: Identifier,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BrokerResponse(Vec<BrokerSingleResponse>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BrokerSingleResponse {
    Error(ServiceError),
    Product(Product),
}
