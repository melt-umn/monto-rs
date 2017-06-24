use futures::{Async, Future, Poll};
use hyper;
use hyper::StatusCode;
use hyper::client::FutureResponse;
use super::{Client, HttpClient};
use url::Url;

/// A Future for a Client negotiating version information and establishing a
/// connection to the Broker.
pub struct NewFuture(Url, HttpClient, FutureResponse);

impl NewFuture {
    pub(crate) fn new(url: Url, client: HttpClient, res: FutureResponse) -> NewFuture {
        NewFuture(url, client, res)
    }
}

impl Future for NewFuture {
    type Item = Client;
    type Error = NewFutureError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.2.poll() {
            Ok(Async::Ready(res)) => match res.status() {
                StatusCode::Ok => unimplemented!(),
                code => Err(NewFutureErrorKind::BadStatus(code).into()),
            },
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err.into()),
        }
    }
}

error_chain! {
    types {
        NewFutureError, NewFutureErrorKind, NewFutureResultExt;
    }
    foreign_links {
        Hyper(hyper::Error)
            #[doc = "An error from the network."];
    }
    errors {
        /// A status other than Ok was received from the Broker, indicating
        /// that the Client is not compatible.
        BadStatus(code: StatusCode) 
    }
}
