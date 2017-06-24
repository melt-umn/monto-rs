extern crate futures;
extern crate monto_broker;
extern crate pretty_env_logger;
extern crate tokio_core;
extern crate tokio_signal;
extern crate toml;

use futures::{Future, Stream};
use monto_broker::Broker;
use tokio_core::reactor::Core;

fn main() {
    // Start logging, or die.
    pretty_env_logger::init().unwrap();

    // Load the configuration.
    let config = monto_broker::config::load_config();

    // Create the I/O loop.
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    // Create the Broker.
    let handle = core.handle();
    let broker = core.run(Broker::new(config, handle))
        .expect("Couldn't initialize Broker");

    // Make a future for Ctrl-C being received (whatever that means on your OS).
    let handle = core.handle();
    let ctrl_c = tokio_signal::ctrl_c(&handle).and_then(|s| {
        s.into_future()
            .map(|_| ())
            .map_err(|(err, _)| err)
    });

    // Run the Broker.
    core.run(broker.serve_until(ctrl_c)).unwrap()
}

