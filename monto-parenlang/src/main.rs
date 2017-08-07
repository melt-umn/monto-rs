extern crate either;
extern crate itertools;
extern crate monto;
extern crate tokio_core;
extern crate void;

mod depth;
mod parenlang;

use either::{Left, Right};
use tokio_core::reactor::Core;
use void::unreachable;

use monto::service::Service;
use monto::service::config::Config;

use depth::DepthProvider;

fn main() {
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
    let err = match core.run(service.serve_forever()) {
        Ok(void) => unreachable(void),
        Err(Left(void)) => unreachable(void),
        Err(Right(e)) => e,
    };
    panic!("{}", err);
}
