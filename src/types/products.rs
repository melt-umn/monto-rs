//! The products defined by [Section
//! 6](https://melt-umn.github.io/monto-v3-draft/draft01/#6-products) of the specfication:
//!
//!  - [`directory`](struct.Directory.html)
//!  - [`errors`](struct.Errors.html)
//!  - [`highlighting`](struct.Highlighting.html)
//!  - [`source`](struct.Source.html)

/// A listing of a directory.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#6-1-directory)
#[derive(Clone, Deserialize, Serialize)]
pub struct Directory(pub Vec<DirectoryEntry>);

/// An entry in a `directory` Product.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryEntry {
    /// The name of the file.
    pub name: String,
    /// The absolute path to the file.
    pub absolute_path: String,
    /// The type of file.
    #[serde(rename = "type")]
    pub entry_type: DirectoryEntryType,
}

/// The type of a file identified by a DirectoryEntry.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum DirectoryEntryType {
    /// A regular file, which can be requested as a `source` Product.
    File,
    /// A directory, which can be requested as a `directory` Product.
    Directory,
    /// A symlink.
    Symlink,
    /// A special file, such as a device node.
    Other,
}

/// Syntactic or semantic errors detected in source code.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#6-2-errors)
#[derive(Clone, Deserialize, Serialize)]
pub struct Errors(pub Vec<Error>);

/// A single error in an `errors` Product.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// The message to display to the user.
    pub message: String,
    /// The start of the marked region. Inclusive.
    pub start_byte: usize,
    /// The end of the marked region. Exclusive.
    pub end_byte: usize,
    /// The severity of the error.
    pub severity: ErrorSeverity,
}

/// The severity of an Error.
#[derive(Clone, Deserialize, Serialize)]
pub enum ErrorSeverity {
    /// An error preventing successful compilation, such as an undefined variable being used.
    Error,
    /// A warning of potential errors in code, such as an unused variable being declared.
    Warning,
    /// An informational message.
    Info,
}

/// Token information to be used for highlighting source code.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#6-3-highlighting)
#[derive(Clone, Deserialize, Serialize)]
pub struct Highlighting(pub Vec<HighlightToken>);

/// A single token in a `highlighting` Product.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightToken {
    /// The start of the marked region. Inclusive.
    pub start_byte: usize,
    /// The end of the marked region. Exclusive.
    pub end_byte: usize,
    /// The type of token to be highlighted.
    pub token: HighlightTokenType,
}

/// The type of a HighlightToken.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum HighlightTokenType {
    /// A comment.
    Comment,
    /// A function name, in languages where this is distinguishable from an identifier.
    Function,
    /// An identifier.
    Identifier,
    /// Keywords, such as `new` or `return` in Java.
    Keyword,
    /// Literal values, such as `12` or `"foo" in C.
    Literal,
    /// Operators.
    Operator,
    /// Non-operator punctuation, such as `{` and `}` in C.
    Punctuation,
    /// A type name.
    Type,
}

/// Source code.
///
/// [Specified here.](https://melt-umn.github.io/monto-v3-draft/draft01/#6-4-source)
pub struct Source(pub String);
