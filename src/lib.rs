//! A crate for the Monto protocol. This crate implements version 3.0.0-draft02 of the protocol,
//! which is specified [here](https://melt-umn.github.io/monto-v3-draft/draft02).

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate hyper;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

pub mod client;
pub mod service;
pub mod common;
