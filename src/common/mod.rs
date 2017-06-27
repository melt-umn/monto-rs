//! Functions and types useful for implementing both the Client and Service
//! Protocols.

pub mod messages;
pub mod products;

use std::marker::PhantomData;

use either::{Either, Left, Right};
use futures::{Async, Future, Poll, Stream};
use futures::future::ok;
use hyper::{Body, Response, StatusCode};
use hyper::Error as HyperError;
use hyper::header::{ContentLength, ContentType};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::error::Error as SerdeError;

/// Deserializes an object as JSON from the request.
pub fn json_request<T: DeserializeOwned + 'static>(body: Body) -> Box<Future<Item=T, Error=Either<HyperError, SerdeError>>> {
    Box::new(body.concat2()
        .map_err(Left)
        .and_then(|bs| serde_json::from_slice(&*bs).map_err(Right)))
}

/// Converts an object to JSON and serves it as a Response.
pub fn json_response<T: Serialize>(t: T, status: StatusCode) -> Box<Future<Item=Response<Body>, Error=HyperError>> {
    let res = serde_json::to_string(&t).unwrap();
    Box::new(ok(Response::new()
        .with_status(status)
        .with_header(ContentLength(res.len() as u64))
        .with_header(ContentType("application/json".parse().unwrap()))
        .with_body(res)))
} 
