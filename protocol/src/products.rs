//! Products types defined by
//! [Section 6](https://melt-umn.github.io/monto-v3-draft/draft03/#6-products)
//! of the Monto specification.

use std::fmt::{Formatter, Result as FmtResult};
use std::fs::FileType;
use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as SerdeError, Unexpected, Visitor};
use serde_json::{to_value, Value};

use {Language, Product, ProductName};

/// A listing of a directory.
///
/// Defined in
/// [Section 6.1](https://melt-umn.github.io/monto-v3-draft/draft03/#6-1-directory)
/// of the specification.
pub struct Directory {
    /// The path at which the directory is present.
    pub path: String,

    /// The entries in the directory.
    pub entries: Vec<DirectoryEntry>,
}

impl From<Directory> for Product {
    fn from(d: Directory) -> Product {
        let value = to_value(d.entries);
        Product {
            name: ProductName::Directory,
            language: Language::None,
            path: d.path,
            value: value.unwrap(),
        }
    }
}

/// A single entry in a directory.
#[derive(Deserialize, Serialize)]
pub struct DirectoryEntry {
    /// The basename of the file.
    pub name: String,

    /// The absolute path to the file.
    pub absolute_path: PathBuf,

    /// The type of the entry.
    #[serde(rename = "type")]
    pub file_type: DirectoryEntryType,
}

/// The type of a directory entry.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd,
         Serialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum DirectoryEntryType {
    /// A regular file.
    File,

    /// A directory.
    Directory,

    /// A symbolic link.
    Symlink,

    /// A special file, such as a device node or TTY.
    Other,
}

impl From<FileType> for DirectoryEntryType {
    fn from(t: FileType) -> DirectoryEntryType {
        if t.is_file() {
            DirectoryEntryType::File
        } else if t.is_dir() {
            DirectoryEntryType::Directory
        } else if t.is_symlink() {
            DirectoryEntryType::Symlink
        } else {
            DirectoryEntryType::Other
        }
    }
}

/// Syntactic or semantic errors detected in source code.
///
/// Defined in
/// [Section 6.2](https://melt-umn.github.io/monto-v3-draft/draft03/#6-2-errors)
/// of the specification.
pub struct Errors {
    /// The errors detected.
    pub errors: Vec<Error>,

    /// The language of the source code.
    pub language: Language,

    /// The path of the file.
    pub path: String,
}

impl From<Errors> for Product {
    fn from(e: Errors) -> Product {
        Product {
            name: ProductName::Errors,
            language: e.language,
            path: e.path,
            value: to_value(e.errors).unwrap(),
        }
    }
}

/// A single syntactic or semantic error.
///
/// Defined in
/// [Section 6.2](https://melt-umn.github.io/monto-v3-draft/draft03/#6-2-errors)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd,
         Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Error {
    /// The error message.
    pub message: String,

    /// The first byte of the error.
    pub start_byte: usize,

    /// The last byte of the error.
    pub end_byte: usize,

    /// The severity of the error.
    pub severity: ErrorSeverity,
}

/// The severity of an error.
///
/// Defined in
/// [Section 6.2](https://melt-umn.github.io/monto-v3-draft/draft03/#6-2-errors)
/// of the specification.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorSeverity {
    /// A fatal error.
    Error,

    /// A warning.
    Warning,

    /// An informational message.
    Info,
}

impl<'de> Deserialize<'de> for ErrorSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ErrorSeverity;
            fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
                write!(fmt, "a valid ErrorSeverity")
            }
            fn visit_str<E: SerdeError>(
                self,
                s: &str,
            ) -> Result<ErrorSeverity, E> {
                match s {
                    "error" => Ok(ErrorSeverity::Error),
                    "warning" => Ok(ErrorSeverity::Warning),
                    "info" => Ok(ErrorSeverity::Info),
                    _ => Err(E::invalid_value(
                        Unexpected::Str(s),
                        &"a valid ErrorSeverity",
                    )),
                }
            }
        }
        deserializer.deserialize_string(V)
    }
}

impl Serialize for ErrorSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            ErrorSeverity::Error => "error",
            ErrorSeverity::Warning => "warning",
            ErrorSeverity::Info => "info",
        })
    }
}

#[test]
fn error_severity_serialize_test() {
    macro_rules! test {
        ($sev:expr, $s:expr) => {
            assert_eq!(to_value($sev).unwrap().to_string(), $s);
        }
    }

    test!(ErrorSeverity::Error, r#""error""#);
    test!(ErrorSeverity::Warning, r#""warning""#);
    test!(ErrorSeverity::Info, r#""info""#);
}

/// Token information to be used for highlighting source code.
///
/// Defined in
/// [Section 6.3](https://melt-umn.github.io/monto-v3-draft/draft03/#6-3-highlighting)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd,
         Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HighlightingToken {
    /// The first byte of the token.
    pub start_byte: usize,

    /// The last byte of the token.
    pub end_byte: usize,

    /// The color to give the token.
    pub color: HighlightingColor,
}

/// The color to highlight a token as.
///
/// Note that the two types of highlighting may be freely mixed.
///
/// Defined in
/// [Section 6.3](https://melt-umn.github.io/monto-v3-draft/draft03/#6-3-highlighting)
/// of the specification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq,
         PartialOrd, Serialize)]
#[serde(content = "value", rename_all = "snake_case", tag = "type")]
pub enum HighlightingColor {
    /// A color from the traditional 16-color ANSI palette, which is
    /// interpreted based on the client's theming.
    ///
    /// This value must be between zero and 15.
    ///
    /// This should be preferred for semantic highlighting.
    Palette(u8),

    /// A specific token type, which is converted to a color by the client.
    ///
    /// This should be preferred for syntax highlighting.
    Token(HighlightingColorToken),
}

/// The type of a token.
///
/// Defined in
/// [Section 6.3](https://melt-umn.github.io/monto-v3-draft/draft03/#6-3-highlighting)
/// of the specification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq,
         PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum HighlightingColorToken {
    /// A comment in source code.
    Comment,

    /// A function name, if possible to discern from a variable name.
    Function,

    /// A variable name.
    Identifier,

    /// A reserved keyword.
    Keyword,

    /// A literal value.
    Literal,

    /// A punctuation operator, such as `+` or `*` in a C-based language.
    Operator,

    /// Miscellaneous punctuation, such as `{` or `}` in a C-based language.
    Punctuation,

    /// A type.
    Type,
}

/// Source code.
///
/// Defined in
/// [Section 6.4](https://melt-umn.github.io/monto-v3-draft/draft03/#6-4-source)
/// of the specification.
pub struct Source {
    /// The contents of the file.
    pub contents: String,

    /// The language of the source code.
    pub language: Language,

    /// The path of the file.
    pub path: String,
}

impl From<Source> for Product {
    fn from(s: Source) -> Product {
        Product {
            name: ProductName::Source,
            language: s.language,
            path: s.path,
            value: Value::String(s.contents),
        }
    }
}
