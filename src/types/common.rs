use regex::Regex;
use semver::Version;
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

/// A reverse-hostname-style dotted identifier.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#3-1-1-identifier)
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Identifier(pub String);

impl Identifier {
    /// Returns whether the identifier is valid or not.
    pub fn is_valid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("[a-zA-Z_][a-zA-Z_0-9]*(\\.[a-zA-Z_][a-zA-Z_0-9]*)*").unwrap();
        }
        RE.is_match(&self.0)
    }
}

/// The version number of the Monto protocol.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#3-1-2-montoversion)
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MontoVersion {
    /// The major version.
    pub major: u64,
    /// The minor version.
    pub minor: u64,
    /// The patch version.
    pub patch: u64,
}

impl Default for MontoVersion {
    fn default() -> MontoVersion {
        MontoVersion {
            major: 3,
            minor: 0,
            patch: 0,
        }
    }
}

impl Display for MontoVersion {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Version::from(*self).fmt(fmt)
    }
}

impl From<MontoVersion> for Version {
    fn from(monto: MontoVersion) -> Version {
        Version {
            major: monto.major,
            minor: monto.minor,
            patch: monto.patch,
            pre: Vec::new(),
            build: Vec::new(),
        }
    }
}

/// A name after a dotted identifier.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#3-1-3-namespacedname)
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NamespacedName(pub String);

impl NamespacedName {
    /// Returns whether the name is valid or not.
    pub fn is_valid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("[a-zA-Z_][a-zA-Z_0-9]*(\\.[a-zA-Z_][a-zA-Z_0-9]*)*/[a-zA-Z_][a-zA-Z_0-9]*").unwrap();
        }
        RE.is_match(&self.0)
    }
}

/// A Product, along with its contents.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#3-1-4-Product)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Product {
    /// The name of the Product.
    pub name: ProductName,
    /// The language corresponding to the Product.
    pub language: Option<Identifier>,
    /// The filesystem path corresponding to the Product.
    pub path: Option<String>,
    /// The contents of the Product.
    pub contents: Value,
}

/// A Product's name and language.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#3-1-5-productidentifier)
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProductIdentifier {
    /// The name of the Product.
    pub name: ProductName,
    /// The language corresponding to the Product.
    pub language: Option<Identifier>,
    /// The filesystem path corresponding to the Product.
    pub path: Option<String>,
}

/// The name of a Product.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#3-1-6-productname)
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ProductName {
    /// A `directory` Product.
    Directory,
    /// An `errors` Product.
    Errors,
    /// A `highlighting` Product.
    Highlighting,
    /// A `source` Product.
    Source,
    /// A vendor-specific Product.
    Other(NamespacedName),
}
