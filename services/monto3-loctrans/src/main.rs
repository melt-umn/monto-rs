//! A Monto service that translates the products given by the ableC service to
//! those given by the Monto specification.

extern crate either;
#[macro_use]
extern crate log;
extern crate monto3_common;
#[macro_use]
extern crate monto3_service;
extern crate pretty_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate void;

mod errors;
mod highlighting;
#[cfg(test)]
mod tests;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use either::{Left, Right};
use monto3_service::Service;
use monto3_service::config::Config;
use tokio_core::reactor::Core;
use void::unreachable;

use errors::Errors;
use highlighting::Highlighting;

fn main() {
    pretty_logger::init_to_defaults().unwrap();
    let mut c = Core::new().unwrap();
    let config = Config::load("monto-loctrans");
    let mut s = Service::new(config, c.handle());

    s.add_provider(Errors);
    s.add_provider(Highlighting);

    let err = match c.run(s.serve_forever()) {
        Ok(void) => unreachable(void),
        Err(Right(err)) => err,
        Err(Left(void)) => unreachable(void),
    };
    error!("{}", err);
}

fn pos_to_byte(
    path: &str,
    start: (usize, usize),
    end: (usize, usize),
) -> Result<(usize, usize), Box<Error>> {
    let buf = {
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        buf
    };
    let s = one_pos_to_byte(&buf, start.0, start.1)?;
    let e = one_pos_to_byte(&buf, end.0, end.1)?;
    Ok((s, e))
}

fn one_pos_to_byte(buf: &str, mut line: usize, mut col: usize) -> Result<usize, Box<Error>> {
    let mut n = 0;
    if line == 0 {
        return Err("Line must not be 0".into());
    }
    line -= 1;
    for c in buf.chars() {
        n += 1;
        if line == 0 {
            if col == 0 {
                return Ok(n);
            } else if c == '\n' {
                return Err("No such position (col)".into())
            } else {
                col -= 1;
            }
        } else if c == '\n' {
            line -= 1;
        }
    }
    if line == 0 && col == 0 {
        Ok(n)
    } else {
        Err("No such position".into())
    }
}
