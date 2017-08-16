//! The Messages specific to the Service Protocol, as described in
//! [Section 5.4](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-service-protocol-messages)
//! of the specification.

use std::collections::BTreeSet;

use common::messages::{GenericProduct, Product, ProductDescriptor, ProductIdentifier, ProtocolVersion, SoftwareVersion, NamespacedName};

/// The Message that a Broker sends to a Service during version negotiation.
///
/// Defined in
/// [Section 5.4.1](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-1-servicebrokernegotiation)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ServiceBrokerNegotiation {
    /// The version of the Service Protocol the Broker supports.
    pub monto: ProtocolVersion,

    /// The version information of the Broker.
    pub broker: SoftwareVersion,

    /// The extensions that are supported by the Broker.
    #[serde(default, skip_serializing_if="BTreeSet::is_empty")]
    pub extensions: BTreeSet<ServiceExtension>,
}

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
    pub products: BTreeSet<ProductDescriptor>,
}

/// An extension to the Service Protocol.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all="snake_case", untagged)]
pub enum ServiceExtension {
    /// An unknown and unsupported extension.
    Unknown(NamespacedName),
}

/// The Message that a Service sends to a Broker during version negotiation.
///
/// Defined in
/// [Section 5.4.3](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-3-brokerrequest)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrokerRequest {
    /// The product being requested.
    pub request: ProductIdentifier,

    /// Products provided with the request.
    pub products: Vec<GenericProduct>,
}

/// Errors encountered by a Service.
///
/// Defined in
/// [Section 5.4.4](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-4-serviceerrors)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceErrors {
    /// The errors encountered.
    pub errors: Vec<ServiceError>,

    /// Any notices generated.
    pub notices: Vec<ServiceNotice>,
}

/// A single error in a ServiceErrors.
///
/// Defined in
/// [Section 5.4.4](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-4-serviceerrors)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(content="value", rename_all="snake_case", tag="type")]
pub enum ServiceError {
    /// An error representing a dependency not being present.
    UnmetDependency(ProductIdentifier),

    /// A miscellaneous error.
    Other(String),
}

/// A response containing a Product from a Service to be returned the Broker.
///
/// Defined in
/// [Section 5.4.5](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-5-serviceproduct)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceProduct<P: Product> {
    /// The product sent.
    pub product: P,

    /// Any notices generated.
    pub notices: Vec<ServiceNotice>,
}

/// A message from a Broker to the Service signalling a non-error special condition.
///
/// Defined in
/// [Section 5.4.6](https://melt-umn.github.io/monto-v3-draft/draft02/#5-4-6-servicenotice)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(content="value", rename_all="snake_case", tag="type")]
pub enum ServiceNotice {
    /// A notice that a dependency was unused when producing a Product.
    UnusedDependency(ProductIdentifier),
}
