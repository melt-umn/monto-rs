//! Lists the products exposed by the Broker.
//!
//! ## Usage:
//!
//! `monto-list-products [BROKER-ADDRESS]`

extern crate log;
extern crate monto;
extern crate simple_logger;
extern crate tokio_core;

use std::env::args;
use std::fmt::Display;
use std::process::exit;

use log::LogLevel;
use tokio_core::reactor::Core;

use monto::client::{Client, Config};

fn main() {
    // Start the logger.
    simple_logger::init_with_level(LogLevel::Info).unwrap();

    // Parse the arguments and create a config based on them.
    // TODO: This should use clap or something.
    let mut args = args()
        .skip(1)
        .collect::<Vec<_>>();
    let config = match args.len() {
        0 => Config::default(),
        1 => Config { host: args.remove(0), ..Config::default() },
        _ => {
            eprintln!("Usage: monto-list-products [BROKER-ADDRESS]");
            exit(-1);
        },
    };

    // Create the main event loop.
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    // Connect to the Broker and list products.
    let client_handle = core.handle();
    let client = must(core.run(Client::new(config, client_handle)));
    for (ident, desc) in client.products() {
        println!("{}: {:?}", ident, desc);
    }
}

fn must<T, E: Display>(r: Result<T, E>) -> T {
    match r {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(-2);
        },
    }
}
