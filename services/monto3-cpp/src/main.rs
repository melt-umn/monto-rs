/// A Monto service for the C preprocessor.
extern crate either;
extern crate futures;
#[macro_use]
extern crate log;
extern crate monto3_common;
#[macro_use]
extern crate monto3_service;
extern crate pretty_logger;
extern crate serde_json;
extern crate tokio_core;
extern crate void;

use std::io::Write;
use std::process::{Command, Stdio};

use either::{Left, Right};
use monto3_common::messages::Language;
use monto3_service::Service;
use monto3_service::config::Config;
use monto3_service::helpers::simple_fn;
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
    (p, ps) => {
        simple_fn(p, ps, Language::C, |src| {
            let mut cpp = Command::new("cpp")
                .arg("-D_POSIX_C_SOURCE")
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|x| format!("Couldn't open cpp: {}", x))?;
            cpp.stdin.as_mut().unwrap().write_all(src.as_bytes())
                .map_err(|x| format!("Couldn't write to cpp: {}", x))?;
            let o = cpp.wait_with_output()
                .map_err(|x| format!("cpp died: {}", x))?;
            if o.status.success() {
                Ok(Value::String(btos(&o.stdout)))
            } else {
                Err(format!("{:#?}", vec![
                    ServiceError::Other(format!("Exited with {:?}", o.status.code())),
                    ServiceError::Other(btos(&o.stdout)),
                    ServiceError::Other(btos(&o.stderr)),
                ]))
            }
        })
    }
}
