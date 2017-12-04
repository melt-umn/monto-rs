#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate monto3_broker;
extern crate pretty_logger;
extern crate tokio_core;
extern crate void;

use tokio_core::reactor::Core;
use void::{ResultVoidExt, unreachable};

use monto3_broker::Broker;
use monto3_broker::config::Config;

fn main() {
    // Start logging.
    pretty_logger::init_to_defaults().unwrap();

    // Load the configuration.
    let config = Config::load_with_args("monto3-broker", crate_version!());
    info!("Using config {:?}", config);

    // Create the I/O loop.
    let mut core = Core::new().expect("Couldn't create event loop");

    // Create the Broker and connect to services.
    let handle = core.handle();
    let broker = core.run(Broker::new(config, handle)).expect(
        "Couldn't initialize Broker",
    );

    // Run the Broker, listening for clients.
    let r = core.run(broker.serve_forever());
    unreachable(r.void_unwrap());
}
