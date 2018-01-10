//! Functions and types useful for implementing both the Client and Service
//! Protocols.

extern crate either;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod messages;
pub mod products;

use either::{Either, Left, Right};
use futures::{Future, Stream};
use futures::future::{err, ok};
use hyper::{Body, Response, StatusCode};
use hyper::Error as HyperError;
use hyper::header::{ContentLength, ContentType};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::error::Error as SerdeError;

/// Creates an error response.
pub fn error_response(
    status: StatusCode,
) -> Box<Future<Item = Response<Body>, Error = HyperError>> {
    let res = status.to_string();
    Box::new(ok(Response::new()
        .with_status(status)
        .with_header(ContentLength(res.len() as u64))
        .with_header(ContentType("text/plain".parse().unwrap()))
        .with_body(res)))
}

/// Deserializes an object as JSON from the request.
pub fn json_request<T: DeserializeOwned + 'static>(
    body: Body,
) -> Box<Future<Item = T, Error = Either<HyperError, SerdeError>>> {
    Box::new(
        body.concat2()
            .map_err(Left)
            .and_then(|bs| serde_json::from_slice(&*bs).map_err(Right)),
    )
}

/// Converts an object to JSON and serves it as a Response.
pub fn json_response<T: Serialize>(
    t: T,
    status: StatusCode,
) -> Box<Future<Item = Response<Body>, Error = Either<HyperError, SerdeError>>>
{
    let res = match serde_json::to_string(&t) {
        Ok(s) => s,
        Err(e) => return Box::new(err(Right(e))),
    };
    Box::new(ok(Response::new()
        .with_status(status)
        .with_header(ContentLength(res.len() as u64))
        .with_header(ContentType("application/json".parse().unwrap()))
        .with_body(res)))
}
