//! Products types defined by
//! [Section 6](https://melt-umn.github.io/monto-v3-draft/draft02/#6-products)
//! of the Monto specification.

use std::fs::FileType;
use std::path::PathBuf;

use serde_json::{to_value, Value};

use common::messages::{Language, ProductName, Product};

/// A listing of a directory.
///
/// Defined in
/// [Section 6.1](https://melt-umn.github.io/monto-v3-draft/draft02/#6-1-directory)
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
    #[serde(rename="type")]
    pub file_type: DirectoryEntryType,
}

/// The type of a directory entry.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all="snake_case", untagged)]
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
/// [Section 6.2](https://melt-umn.github.io/monto-v3-draft/draft02/#6-2-errors)
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
/// [Section 6.2](https://melt-umn.github.io/monto-v3-draft/draft02/#6-2-errors)
/// of the specification.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all="snake_case")]
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
/// [Section 6.2](https://melt-umn.github.io/monto-v3-draft/draft02/#6-2-errors)
/// of the specification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all="snake_case", untagged)]
pub enum ErrorSeverity {
    /// A fatal error.
    Error,

    /// A warning.
    Warning,

    /// An informational message.
    Info,
}

/// Source code.
///
/// Defined in
/// [Section 6.4](https://melt-umn.github.io/monto-v3-draft/draft02/#6-4-source)
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
