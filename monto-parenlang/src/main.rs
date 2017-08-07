extern crate either;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate monto;
extern crate simple_logger;
extern crate tokio_core;
extern crate void;

mod depth;
mod parenlang;

use either::{Left, Right};
use log::LogLevel;
use tokio_core::reactor::Core;
use void::unreachable;

use monto::service::Service;
use monto::service::config::Config;

use depth::DepthProvider;

fn main() {
    // Start the logger.
    simple_logger::init_with_level(LogLevel::Info).unwrap();

    // Create the main I/O loop.
    let mut core = Core::new()
        .expect("Couldn't create event loop");

    // Create the service value.
    let config = Config::load("monto-parenlang");
    let handle = core.handle();
    let mut service = Service::new(config, handle);

    // Add a fictitious "depth" product.
    service.add_provider(DepthProvider);

    // Run the service forever.
    info!("asdf");
    let err = match core.run(service.serve_forever()) {
        Ok(void) => unreachable(void),
        Err(Left(void)) => unreachable(void),
        Err(Right(e)) => e,
    };
    panic!("{}", err);
}
