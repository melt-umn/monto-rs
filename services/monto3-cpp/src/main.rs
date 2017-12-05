/// A Monto service for the C preprocessor.
extern crate either;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate monto3_service;
extern crate pretty_logger;
extern crate serde_json;
extern crate tokio_core;
extern crate void;

use std::process::Command;

use either::{Left, Right};
use monto3_service::Service;
use monto3_service::config::Config;
use monto3_service::messages::ServiceError;
use serde_json::Value;
use tokio_core::reactor::Core;
use void::unreachable;

fn main() {
    pretty_logger::init_to_defaults().unwrap();
    let mut c = Core::new().unwrap();
    let config = Config::load("monto-cpp");
    let mut s = Service::new(config, c.handle());

    s.add_provider(Cpp);

    let err = match c.run(s.serve_forever()) {
        Ok(void) => unreachable(void),
        Err(Right(err)) => err,
        Err(Left(void)) => unreachable(void),
    };
    error!("{}", err);
}

fn btos(b: &[u8]) -> String {
    String::from_utf8_lossy(b).to_string()
}

simple_service_provider! {
    name = Cpp;
    product = "edu.umn.cs.melt.preprocessed_source";
    language = "c";
    (p, _ps) => {
        let r = Command::new("cpp")
            .arg("-D_POSIX_C_SOURCE")
            .arg(p)
            .output();
        let r = match r {
            Ok(o) => if o.status.success() {
                Ok(Value::String(btos(&o.stdout)))
            } else {
                Err(vec![
                    ServiceError::Other(format!("Exited with {:?}", o.status.code())),
                    ServiceError::Other(btos(&o.stdout)),
                    ServiceError::Other(btos(&o.stderr)),
                ])
            },
            Err(e) => Err(vec![
                ServiceError::Other(e.to_string())
            ]),
        };
        (r, Vec::new())
    }
}
