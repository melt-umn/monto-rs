//! A crate for the Monto protocol. This crate implements version 3.0.0-draft01 of the protocol,
//! which is specified [here](https://melt-umn.github.io/monto-v3-draft/draft01).

// #![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

pub mod client;
pub mod broker;
pub mod service;
pub mod types;
