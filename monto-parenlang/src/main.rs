extern crate monto;

use monto::service::Service;
use monto::service::config::Config;

fn main() {
    let config = Config::load("monto-parenlang");
    let service = Service::new(config);
    println!("Hello, world!");
    loop {
        std::thread::park();
    }
}
