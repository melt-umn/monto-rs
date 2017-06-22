//! Types useful for implementing a Monto Broker.

use super::Identifier;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BrokerVersion {
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
