use hyper::StatusCode;

use monto3_common::json_response;
use monto3_protocol::client::ClientNegotiation;

use client::{BoxedFuture, Client};

impl Client {
    /// Performs negotiation.
    pub fn negotiation(self, cn: ClientNegotiation) -> BoxedFuture {
        debug!("Got ClientNegotiation {:?}", cn);
        let broker = self.0.borrow();

        let cbn = broker.client_negotiation();
        let status = if cbn.monto.compatible(&cn.monto) {
            StatusCode::Ok
        } else {
            StatusCode::BadRequest
        };

        json_response(cbn, status)
    }
}
