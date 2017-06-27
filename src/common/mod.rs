//! Functions and types useful for implementing both the Client and Service
//! Protocols.

pub mod messages;
pub mod products;

use either::{Either, Left, Right};
use futures::{Future, Stream};
use hyper::{Body, Response, StatusCode};
use hyper::Error as HyperError;
use hyper::header::{ContentLength, ContentType};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::error::Error as SerdeError;

/// Deserializes an object as JSON from the request.
pub fn json_request<'de, T: Deserialize<'de>>(body: Body) -> Box<Future<Item=T, Error=Either<HyperError, SerdeError>>> {
    let f = body
        .fold(Vec::new(), |v, c| { v.extend(c); Ok(v) })
        .then(|r| match r {
            Ok(b) => match serde_json::from_slice(&b) {
                Ok(v) => Ok(v),
                Err(e) => Err(Right(e)),
            },
            Err(e) => Err(Left(e)),
        });
    Box::new(f)
}

/// Converts an object to JSON and serves it as a Response.
pub fn json_response<T: Serialize>(t: T, status: StatusCode) -> Box<Future<Item=Response<Body>, Error=HyperError>> {
    let res = serde_json::to_string(&t).unwrap();
    Box::new(Ok(Response::new()
        .with_status(status)
        .with_header(ContentLength(res.len() as u64))
        .with_header(ContentType("application/json".parse().unwrap()))
        .with_body(res)))
} 
