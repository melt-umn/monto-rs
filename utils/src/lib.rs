//! Utilities for implementing both the Client and Service Protocols.
//!
//! Parts of this might later get factored out into a general-purpose "strongly
//! typed API" library -- I'm really liking the concept of encoding a network
//! protocol into the type system.
//!
//! TODO: Work around needing specialization.
#![feature(specialization)]
#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate monto3_protocol;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

mod haskellism;
pub mod http_client;
