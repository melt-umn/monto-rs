//! Functions and types useful for implementing both the Client and Service
//! Protocols.

pub mod messages;
pub mod products;

use futures::Future;
use futures::future::ok;
use hyper::{Body, Response, StatusCode};
use hyper::Error as HyperError;
use hyper::header::{ContentLength, ContentType};
use serde::Serialize;
use serde_json;

/// Converts an object to JSON and serves it as a Response.
pub fn json_response<T: Serialize>(t: T, status: StatusCode) -> Box<Future<Item=Response<Body>, Error=HyperError>> {
    let res = serde_json::to_string(&t).unwrap();
    Box::new(ok(Response::new()
        .with_status(status)
        .with_header(ContentLength(res.len() as u64))
        .with_header(ContentType("application/json".parse().unwrap()))
        .with_body(res)))
} 
