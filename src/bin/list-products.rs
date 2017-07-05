//! Lists the products exposed by the Broker.
//!
//! ## Usage:
//!
//! `monto-list-products [BROKER-ADDRESS]`

extern crate monto;
extern crate tokio_core;

use std::env::args;
use std::io::{stderr, Write};
use std::process::exit;

use tokio_core::reactor::Core;

use monto::client::{Client, Config};

fn main() {
    // Create the main event loop.
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    // Parse the arguments and create a config based on them.
    // TODO: This should use clap or something.
    let mut args = args()
        .skip(1)
        .collect::<Vec<_>>();
    let config = match args.len() {
        0 => Config::default(),
        1 => Config { host: args.remove(0), ..Config::default() },
        _ => {
            writeln!(stderr(), "Usage: monto-list-products [BROKER-ADDRESS]").unwrap();
            exit(-1);
        },
    };

    // Connect to the Broker and list products.
    let client_handle = core.handle();
    match core.run(Client::new(config, client_handle)) {
        Ok(client) => println!("{:?}", client.products().collect::<Vec<_>>()),
        Err(err) => {
            writeln!(stderr(), "Error: {}", err).unwrap();
            exit(-2);
        },
    }
}
