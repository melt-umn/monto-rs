extern crate futures;
#[macro_use]
extern crate log;
extern crate monto;
extern crate pretty_env_logger;
extern crate tokio_core;
extern crate toml;
extern crate void;

use monto::broker::Broker;
use monto::broker::config::Config;
use tokio_core::reactor::Core;
use void::{ResultVoidExt, unreachable};

fn main() {
    // Start logging, or die.
    pretty_env_logger::init().unwrap();

    // Load the configuration.
    let config = Config::load();
    info!("Using config {:?}", config);

    // Create the I/O loop.
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    // Create the Broker and connect to services.
    let handle = core.handle();
    let broker = core.run(Broker::new(config, handle))
        .expect("Couldn't initialize Broker");

    // Run the Broker, listening for clients.
    let r = core.run(broker.serve_forever());
    unreachable(r.void_unwrap());
}

