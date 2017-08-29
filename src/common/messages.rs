//! The Messages common to both the Client and Service Protocols, as described in
//! [Section 3.1](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-common-messages)
//! of the specification.

use std::cmp::Ordering;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;
use semver::Version as SemverVersion;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as SerdeError, Visitor};
use serde_json::Value;

/// A reverse-hostname-style dotted identifier, which must have at least two components.
///
/// Defined in
/// [Section 3.1.1](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-1-identifier)
/// of the specification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier {
	namespace: Vec<String>,
	name: String,
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{Error, Unexpected, Visitor};

        struct IdentifierVisitor;
        impl<'de> Visitor<'de> for IdentifierVisitor {
            type Value = Identifier;
            fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
                write!(fmt, "a reverse-hostname-style dotted identifier with at least two components")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                lazy_static! {
                    static ref PART: Regex = Regex::new("[a-zA-Z_][a-zA-Z_0-9]*").unwrap();
                }

                let mut parts = v.split('.').map(str::to_owned).collect::<Vec<_>>();
                if parts.len() < 2 || parts.iter().any(|p| !PART.is_match(p)) {
                    return Err(Error::invalid_value(Unexpected::Str(v), &self));
                }

                let name = parts.pop().unwrap();
                Ok(Identifier {
                    name,
                    namespace: parts,
                })
            }
        }

        d.deserialize_string(IdentifierVisitor)
    }
}

impl Display for Identifier {
	fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
		for c in &self.namespace {
			write!(fmt, "{}.", c)?;
		}
		write!(fmt, "{}", self.name)
	}
}

impl FromStr for Identifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("[a-zA-Z_][a-zA-Z_0-9]*(\\.[a-zA-Z_][a-zA-Z_0-9]*)+").unwrap();
        }
        if RE.is_match(s) {
            let mut parts = s.split('.').map(str::to_owned).collect::<Vec<_>>();
            let name = parts.pop().unwrap().to_owned();
            Ok(Identifier {
                namespace: parts,
                name: name,
            })
        } else {
            Err(())
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(s)
    }
}

/// The programming language associated with a Product.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Language {
    /// JSON, as described by [RFC 7159](https://tools.ietf.org/html/rfc7159).
    Json,

    /// The language of plain UTF-8 text.
    Text,

    /// The language of directories, and any other Product that does not
    /// inherently have a language.
    None,

    /// A language not otherwise present in this enumeration.
    Other(String),
}

impl Language {
    /// The name of the language, as a string.
    fn name(&self) -> &str {
        match *self {
            Language::Json => "json",
            Language::Text => "text",
            Language::None => "none",
            Language::Other(ref name) => name,
        }
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Language;
            fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
                write!(fmt, "a string")
            }
            fn visit_str<E: SerdeError>(self, s: &str) -> Result<Language, E> {
                self.visit_string(s.to_string())
            }
            fn visit_string<E: SerdeError>(self, s: String) -> Result<Language, E> {
                Ok(s.into())
            }
        }
        deserializer.deserialize_string(V)
    }
}

impl Display for Language {
	fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.name())
	}
}

impl From<String> for Language {
    fn from(s: String) -> Language {
        match s.as_ref() {
            "json" => Language::Json,
            "text" => Language::Text,
            "none" => Language::None,
            _ => Language::Other(s),
        }
    }
}

impl Serialize for Language {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

/// A name after a dotted identifier.
///
/// Defined in
/// [Section 3.1.2](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-2-namespacedname)
/// of the specification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NamespacedName {
    namespace: Identifier,
    name: String,
}

impl<'de> Deserialize<'de> for NamespacedName {
    fn deserialize<D: Deserializer<'de>>(_d: D) -> Result<Self, D::Error> {
        unimplemented!()
    }
}

impl Display for NamespacedName {
	fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
		write!(fmt, "{}/{}", self.namespace, self.name)
	}
}

impl Serialize for NamespacedName {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(s)
    }
}

/// A Product, along with its contents.
///
/// Defined in
/// [Section 3.1.3](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-3-product)
/// of the specification.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Product {
    /// The name of the Product.
    pub name: ProductName,

    /// The language of the Product.
    pub language: Language,

    /// The path of the Product.
    pub path: String,

    /// The contents of the Product.
    pub value: Value
}

/// A Product's name and language.
///
/// Defined in
/// [Section 3.1.4](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-4-productdescriptor)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProductDescriptor {
    /// The name of the Product.
    pub name: ProductName,

    /// The language of the Product.
    pub language: Language,
}

impl From<ProductIdentifier> for ProductDescriptor {
    fn from(p: ProductIdentifier) -> ProductDescriptor {
        ProductDescriptor {
            name: p.name,
            language: p.language,
        }
    }
}

/// A Product's name, language, and path.
///
/// Defined in
/// [Section 3.1.5](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-5-productidentifier)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProductIdentifier {
    /// The name of the Product.
    pub name: ProductName,

    /// The language of the Product.
    pub language: Language,

    /// The path of the Product.
    pub path: String,
}

impl From<Product> for ProductIdentifier {
    fn from(p: Product) -> ProductIdentifier {
        ProductIdentifier {
            name: p.name,
            language: p.language,
            path: p.path,
        }
    }
}

impl<'a> From<&'a Product> for ProductIdentifier {
    fn from(p: &'a Product) -> ProductIdentifier {
        ProductIdentifier {
            name: p.name.clone(),
            language: p.language.clone(),
            path: p.path.clone(),
        }
    }
}

/// The name of a Product.
///
/// Defined in
/// [Section 3.1.6](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-6-productname)
/// of the specification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProductName {
    /// A listing of a directory.
    Directory,

    /// Syntactic or semantic errors detected in source code.
    Errors,

    /// Token information to be used for highlighting source code.
    Highlighting,

    /// Source code.
    Source,

    // TODO other built-in product types

    /// A vendor-specific product.
    Other(Identifier),
}

impl<'de> Deserialize<'de> for ProductName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ProductName;
            fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
                write!(fmt, "an identifier")
            }
            fn visit_str<E: SerdeError>(self, s: &str) -> Result<ProductName, E> {
                s.parse().map_err(|()| {
                    E::custom("not an identifier")
                })
            }
        }
        deserializer.deserialize_str(V)
    }
}

impl Display for ProductName {
	fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            ProductName::Directory => write!(fmt, "directory"),
            ProductName::Errors => write!(fmt, "errors"),
            ProductName::Highlighting => write!(fmt, "highlighting"),
            ProductName::Source => write!(fmt, "source"),
            ProductName::Other(ref ident) => ident.fmt(fmt),
        }
	}
}

impl FromStr for ProductName {
    type Err = <Identifier as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "directory" => Ok(ProductName::Directory),
            "errors" => Ok(ProductName::Errors),
            "highlighting" => Ok(ProductName::Highlighting),
            "source" => Ok(ProductName::Source),
            _ => s.parse().map(ProductName::Other),
        }
    }
}

impl Serialize for ProductName {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

/// The version number of the Client or Server Protocol.
///
/// Defined in
/// [Section 3.1.7](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-7-protocolversion)
/// of the specification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ProtocolVersion {
    /// The major version number.
    pub major: u64,

    /// The minor version number.
    pub minor: u64,

    /// The patch version number.
    pub patch: u64,
}

impl ProtocolVersion {
    /// Creates a new ProtocolVersion.
    pub fn new(major: u64, minor: u64, patch: u64) -> ProtocolVersion {
        ProtocolVersion {
            major, minor, patch,
        }
    }

    /// Returns whether the two protocol versions are compatible. The lowest
    /// (by the Ord implementation) is the one to use.
    pub fn compatible(&self, other: &ProtocolVersion) -> bool {
        self.major == other.major
    }
}

impl Display for ProtocolVersion {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<ProtocolVersion> for SemverVersion {
    fn from(v: ProtocolVersion) -> SemverVersion {
        SemverVersion {
            major: v.major,
            minor: v.minor,
            patch: v.patch,
            build: Vec::new(),
            pre: Vec::new(),
        }
    }
}

impl Ord for ProtocolVersion {
    fn cmp(&self, other: &ProtocolVersion) -> Ordering {
        let l: SemverVersion = self.clone().into();
        let r: SemverVersion = other.clone().into();
        l.cmp(&r)
    }
}

impl PartialOrd for ProtocolVersion {
    fn partial_cmp(&self, other: &ProtocolVersion) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The version and implementation of a Client, Broker, or Service.
///
/// Defined in
/// [Section 3.1.8](https://melt-umn.github.io/monto-v3-draft/draft02/#3-1-8-softwareversion)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SoftwareVersion {
    /// The identifier of the Client, Broker, or Service.
    pub id: Identifier,

    /// The human-readable name of the Client, Broker, or Service.
    #[serde(default)]
    pub name: Option<String>,

    /// The human-readable name of the vendor of the Client, Broker, or Service.
    #[serde(default)]
    pub vendor: Option<String>,

    /// The major version number.
    #[serde(default)]
    pub major: u64,

    /// The minor version number.
    #[serde(default)]
    pub minor: u64,

    /// The patch version number.
    #[serde(default)]
    pub patch: u64,
}

impl Display for SoftwareVersion {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.id)?;
        match (&self.name, &self.vendor) {
            (&Some(ref name), &Some(ref vendor)) => write!(fmt, " ({} by {})", name, vendor)?,
            (&Some(ref name), &None) => write!(fmt, " ({})", name)?,
            (&None, &Some(ref vendor)) => write!(fmt, " by {}", vendor)?,
            (&None, &None) => {},
        }
        write!(fmt, " {}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<SoftwareVersion> for SemverVersion {
    fn from(v: SoftwareVersion) -> SemverVersion {
        SemverVersion {
            major: v.major,
            minor: v.minor,
            patch: v.patch,
            build: Vec::new(),
            pre: Vec::new(),
        }
    }
}
