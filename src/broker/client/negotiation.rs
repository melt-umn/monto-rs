use hyper::StatusCode;

use common::json_response;
use client::messages::ClientNegotiation;
use super::{BoxedFuture, Client};

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
