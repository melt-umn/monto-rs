use std::error::Error as StdError;

use monto3_protocol::Language;
use monto3_protocol::products::{Error, ErrorSeverity};
use monto3_protocol::service::ServiceError;
use monto3_service::helpers::one_to_one_fn;
use serde_json::{from_value, to_value};

use pos_to_byte;

simple_service_provider! {
    name = Errors;
    product = "errors";
    language = "c";
    (p, ps) => {
        one_to_one_fn(p, ps, "edu.umn.cs.melt.ablec.errors".parse().unwrap(), Language::C, |errs| {
            match from_value(errs) {
                Ok(errs) => {
                    let errs: Vec<MeltError> = errs;
                    let r = errs.into_iter()
                        .map(|e| e.convert(p))
                        .collect::<Result<Vec<Error>, _>>();
                    let r = match r {
                        Ok(x) => match to_value(x) {
                            Ok(x) => Ok(x),
                            Err(e) =>Err(vec![ServiceError::Other(e.to_string())]),
                        }
                        Err(e) =>Err(vec![ServiceError::Other(e.to_string())]),
                    };
                    (r, vec![])
                }
                Err(e) => (Err(vec![ServiceError::Other(e.to_string())]), vec![]),
            }
        })
    }
}

#[derive(Deserialize)]
struct MeltError {
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
    message: String,
    severity: ErrorSeverity,
}

impl MeltError {
    fn convert(self, path: &str) -> Result<Error, Box<StdError>> {
        let (s, e) = pos_to_byte(
            path,
            (self.start_line, self.start_col),
            (self.end_line, self.end_col),
        )?;
        Ok(Error {
            message: self.message,
            severity: self.severity,
            start_byte: s,
            end_byte: e,
        })
    }
}
