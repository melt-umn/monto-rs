//! Fetches all possible products for a given file.
//!
//! ## Usage:
//!
//! `monto3-get-products-for FILENAME [--broker BROKER-ADDRESS]`

#[macro_use]
extern crate clap;
extern crate monto3;
extern crate tokio_core;

// use futures::Future;
use monto3::client::{Client, Config};
use tokio_core::reactor::Core;

// TODO Implement error handling.

fn main() {
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    let config = Config::default();
    let client_handle = core.handle();
    let client = core.run(Client::new(config, client_handle))
        .expect("Couldn't connect to broker");

    let services = client.services();
    println!("{:?}", services);
}
