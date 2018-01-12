use std::collections::HashSet;
use std::fmt::Result as FmtResult;

use hyper::error::Result as HyperResult;
use hyper::header::{Formatter, Header, Raw};

use monto3_protocol::{ProtocolExtension, ProtocolVersion};

/// The `Monto-Extension` header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MontoExtension<E: ProtocolExtension>(pub HashSet<E>);

impl<E: ProtocolExtension> Header for MontoExtension<E> {
    fn header_name() -> &'static str {
        "Monto-Extension"
    }

    fn parse_header(raw: &Raw) -> HyperResult<MontoExtension<E>> {
        unimplemented!()
    }

    fn fmt_header(&self, f: &mut Formatter) -> FmtResult {
        self.0.iter().map(|ext| f.fmt_line(ext)).collect()
    }
}

/// The `Monto-Version` header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MontoVersion(pub ProtocolVersion);

impl Header for MontoVersion {
    fn header_name() -> &'static str {
        "Monto-Version"
    }

    fn parse_header(raw: &Raw) -> HyperResult<MontoVersion> {
        unimplemented!()
    }

    fn fmt_header(&self, f: &mut Formatter) -> FmtResult {
        f.fmt_line(&self.0)
    }
}
