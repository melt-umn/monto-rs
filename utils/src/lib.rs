//! Utilities for implementing both the Client and Service Protocols.
#![warn(missing_docs)]

extern crate either;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate monto3_protocol;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;
extern crate void;

pub mod http_client;
