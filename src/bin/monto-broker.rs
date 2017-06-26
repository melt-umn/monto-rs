extern crate futures;
extern crate monto;
extern crate pretty_env_logger;
extern crate tokio_core;
extern crate toml;

use futures::{Future, Stream};
use monto::broker::Broker;
use monto::broker::config::Config;
use tokio_core::reactor::Core;

fn main() {
    // Start logging, or die.
    pretty_env_logger::init().unwrap();

    // Load the configuration.
    let config = Config::load();

    // Create the I/O loop.
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    // Create the Broker and connect to services.
    let handle = core.handle();
    let broker = core.run(Broker::new(config, handle))
        .expect("Couldn't initialize Broker");

    // Run the Broker, listening for clients.
    broker.run_forever()
}

