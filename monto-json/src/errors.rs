use monto::common::messages::{Language, Product, ProductDescriptor, ProductName};
use monto::common::products::Errors;
use monto::service::ServiceFn;
use monto::service::messages::ServiceErrors;

/// Error Checking for JSON. We just parse it with serde_json and try to
/// convert back the errors it gives.
pub fn error_check() -> Result<(), ()> {
    unimplemented!()
}
