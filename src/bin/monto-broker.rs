#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate monto;
extern crate simple_logger;
extern crate tokio_core;
extern crate void;

use log::LogLevel;
use tokio_core::reactor::Core;
use void::{ResultVoidExt, unreachable};

use monto::broker::Broker;
use monto::broker::config::Config;

fn main() {
    // Start logging, or die.
    simple_logger::init_with_level(LogLevel::Info).unwrap();

    // Load the configuration.
    let config = Config::load_with_args("monto-broker", crate_version!());
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
