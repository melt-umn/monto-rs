//! Products types defined by
//! [Section 6](https://melt-umn.github.io/monto-v3-draft/draft02/#6-products)
//! of the Monto specification.

use common::messages::{Identifier, ProductName, ProductValue};
use serde_json::{from_value, to_value, Value};
use std::fs::{DirEntry, FileType};
use std::io::Error as IoError;
use std::path::PathBuf;

/// A listing of a directory.
///
/// Defined in
/// [Section 6.1](https://melt-umn.github.io/monto-v3-draft/draft02/#6-1-directory)
/// of the specification.
#[derive(Deserialize, Serialize)]
pub struct Directory(Vec<DirectoryEntry>);

impl ProductValue for Directory {
    fn name() -> ProductName { ProductName::Directory }
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
#[derive(Deserialize, Serialize)]
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

/// Source code.
///
/// Defined in
/// [Section 6.4](https://melt-umn.github.io/monto-v3-draft/draft02/#6-4-source)
/// of the specification.
#[derive(Deserialize, Serialize)]
pub struct Source(String);

impl ProductValue for Source {
    fn name() -> ProductName { ProductName::Source }
}
