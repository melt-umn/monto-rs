// #[macro_use]
// extern crate clap;
extern crate futures;
extern crate hyper;
extern crate monto;
// #[macro_use]
// extern crate serde_derive;
// extern crate toml;

use futures::future;
use futures::future::FutureResult;
use hyper::StatusCode;
use hyper::server::{Http, Request, Response, Service};

fn main() {
    let addr = "[::]:28888".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(MontoBroker)).unwrap();
    server.run().unwrap();
}

struct MontoBroker;

impl Service for MontoBroker {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match (req.method(), req.path()) {
            _ => {
                let mut res = Response::new();
                res.set_status(StatusCode::NotFound);
                future::ok(res)
            },
        }
    }
}
