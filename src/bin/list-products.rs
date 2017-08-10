//! Lists the products exposed by the Broker.
//!
//! ## Usage:
//!
//! `monto-list-products [BROKER-ADDRESS]`

extern crate itertools;
extern crate log;
extern crate monto;
extern crate pretty_logger;
extern crate tokio_core;

use std::env::args;
use std::fmt::Display;
use std::process::exit;

use itertools::Itertools;
use tokio_core::reactor::Core;

use monto::client::{Client, Config};

fn main() {
    // Start the logger.
    pretty_logger::init_to_defaults().unwrap();

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
    let products = client.products()
        .map(|(i, d)| (i.to_string(), d.language.to_string(), d.name.to_string()))
        .sorted()
        .into_iter()
        .group_by(|&(ref s, _, _)| s.clone());
    for (service, rest) in products.into_iter() {
        println!("{}", service);
        for (lang, rest) in rest.group_by(|&(_, ref l, _)| l.clone()).into_iter() {
            println!("\t{}", lang);
            for (_, _, product) in rest {
                println!("\t\t{}", product);
            }
        }
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
