//! The Messages specific to the Client Protocol, as described in
//! [Section 4.5](https://melt-umn.github.io/monto-v3-draft/draft03/#4-5-client-protocol-messages)
//! of the specification.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use monto3_common::messages::{Identifier, NamespacedName, ProductIdentifier, ProtocolVersion,
                              SoftwareVersion};
use monto3_service::messages::ServiceNegotiation;

/// The Message that a Client sends to a Broker during version negotiation.
///
/// Defined in
/// [Section 4.5.1](https://melt-umn.github.io/monto-v3-draft/draft03/#4-5-1-clientnegotiation)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientNegotiation {
    /// The version of the Client Protocol the Client supports.
    pub monto: ProtocolVersion,

    /// The version information of the Client.
    pub client: SoftwareVersion,

    /// The extensions that are supported by the Client.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub extensions: BTreeSet<ClientExtension>,
}

/// The Message that a Broker sends to a Client during version negotiation.
///
/// Defined in
/// [Section 4.5.2](https://melt-umn.github.io/monto-v3-draft/draft03/#4-5-2-clientbrokernegotiation)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientBrokerNegotiation {
    /// The version of the Client Protocol the Broker supports.
    pub monto: ProtocolVersion,

    /// The version information of the Broker.
    pub broker: SoftwareVersion,

    /// The extensions that are supported by the Broker.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub extensions: BTreeSet<ClientExtension>,

    /// The services the Broker has connected to.
    pub services: BTreeSet<ServiceNegotiation>,
}

/// An extension to the Client Protocol.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum ClientExtension {
    /// An unknown and unsupported extension.
    Unknown(NamespacedName),
}

/// An error that occurs during the sending of a product from a Client to the Broker.
///
/// Defined in
/// [Section 4.5.3](https://melt-umn.github.io/monto-v3-draft/draft03/#4-5-3-brokerputerror)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(content = "value", rename_all = "snake_case", tag = "type")]
pub enum BrokerPutError {
    /// A language was not provided, and it could not be detected by the Broker.
    NoLanguage,
}

impl Display for BrokerPutError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            BrokerPutError::NoLanguage => fmt.write_str(
                "No language was provided, and it could not be detected by the Broker.",
            ),
        }
    }
}

impl Error for BrokerPutError {
    fn description(&self) -> &str {
        match *self {
            BrokerPutError::NoLanguage => {
                "No language was provided, and it could not be detected by the Broker."
            }
        }
    }
}

/// An error that occurs during the requesting of a product by a Client.
///
/// Defined in
/// [Section 4.5.4](https://melt-umn.github.io/monto-v3-draft/draft03/#4-5-4-brokergeterror)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(content = "value", rename_all = "snake_case", tag = "type")]
pub enum BrokerGetError {
    /// A Product was requested from a nonexistent Service.
    NoSuchService,

    /// A Product was requested that the Service does not expose.
    NoSuchProduct,

    /// An error from a Service.
    ServiceError {
        /// The service an error occurred while trying to connect to.
        service: Identifier,

        /// The error that occurred, as described by the Service.
        error: String,
    },

    /// An error trying to connect to a Service.
    ServiceConnectError {
        /// The service an error occurred while trying to connect to.
        service: Identifier,

        /// The error that occurred, as described by the Broker.
        error: String,
    },

    /// A dependency was unresolvable.
    Unresolvable(ProductIdentifier),
}

impl Display for BrokerGetError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            BrokerGetError::NoSuchService => {
                fmt.write_str("A Product was requested from a nonexistent Service")
            }
            BrokerGetError::NoSuchProduct => {
                fmt.write_str("A Product was requested that the Service does not expose")
            }
            BrokerGetError::ServiceError {
                ref service,
                ref error,
            } => write!(fmt, "From service {}: {}", service, error),
            BrokerGetError::ServiceConnectError {
                ref service,
                ref error,
            } => write!(fmt, "When connecting to service {}: {}", service, error),
            BrokerGetError::Unresolvable(ref pi) => {
                write!(fmt, "A product was unresolvable: {:?}", pi)
            }
        }
    }
}

impl Error for BrokerGetError {
    fn description(&self) -> &str {
        match *self {
            BrokerGetError::NoSuchService => "A Product was requested from a nonexistent Service",
            BrokerGetError::NoSuchProduct => {
                "A Product was requested that the Service does not expose"
            }
            BrokerGetError::ServiceError { .. } => "An error from a service",
            BrokerGetError::ServiceConnectError { .. } => "An error trying to connect to a Service",
            BrokerGetError::Unresolvable(_) => "A product was unresolvable",
        }
    }
}
