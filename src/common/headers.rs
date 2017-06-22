//! HTTP headers used by Monto.

header! {
    /// The Monto-Extensions header.
    (MontoExtensions, "Monto-Extensions") => (String)*
}

header! {
    /// The Monto-Version header.
    (MontoVersion, "Monto-Version") => [String]
}
