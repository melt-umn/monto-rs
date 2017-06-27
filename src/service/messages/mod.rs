//! The Messages specific to the Service Protocol, as described in
//! [Section 5.4](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-service-protocol-messages)
//! of the specification.

use common::messages::{ProductIdentifier, ProtocolVersion, SoftwareVersion, NamespacedName};
use std::collections::BTreeSet;

/// The Message that a Service sends to a Broker during version negotiation.
///
/// Defined in
/// [Section 5.4.2](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-2-servicenegotiation)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ServiceNegotiation {
    /// The version of the Service Protocol the Service supports.
    pub monto: ProtocolVersion,

    /// The version information of the Service.
    pub service: SoftwareVersion,

    /// The extensions that are supported by the Service.
    #[serde(default, skip_serializing_if="BTreeSet::is_empty")]
    pub extensions: BTreeSet<ServiceExtension>,

    /// The Products the Service can produce.
    pub products: BTreeSet<ProductIdentifier>,
}

/// An extension to the Service Protocol.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all="snake_case", untagged)]
pub enum ServiceExtension {
    /// An unknown and unsupported extension.
    Unknown(NamespacedName),
}
