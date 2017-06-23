//! Fetches all possible products for a given file.
//!
//! ## Usage:
//!
//! `monto-get-products-for FILENAME [--broker BROKER-ADDRESS]`

// #[macro_use]
// extern crate clap;
extern crate monto;
extern crate tokio_core;

use monto::client::{Client, Config};
use tokio_core::reactor::Core;

// TODO Implement error handling.

fn main() {
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    let config = Config::default();
    let client_handle = core.handle();
    let client = core.run(Client::new(config, client_handle).unwrap())
        .expect("Couldn't connect to broker");

    let products = client.products();
    println!("{:?}", products.collect::<Vec<_>>());
}
