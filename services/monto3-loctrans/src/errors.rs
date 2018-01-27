use std::error::Error as StdError;

use monto3_common::messages::Language;
use monto3_common::products::{Error, ErrorSeverity};
use serde_json::{from_value, to_value};

use pos_to_byte;
use util::product_and_src;

simple_service_provider! {
    name = Errors;
    product = "errors";
    language = "c";
    (p, ps) => {
        product_and_src(p, ps, "edu.umn.cs.melt.ablec.errors".parse().unwrap(), Language::C, |errs, src| {
            match from_value(errs) {
                Ok(errs) => {
                    let errs: Vec<MeltError> = errs;
                    let r = errs.into_iter()
                        .map(|e| e.convert(&src))
                        .collect::<Result<Vec<Error>, _>>();
                    match r {
                        Ok(x) => match to_value(x) {
                            Ok(x) => Ok(x),
                            Err(e) =>Err(e.to_string()),
                        }
                        Err(e) =>Err(e.to_string()),
                    }
                }
                Err(e) => Err(e.to_string()),
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
    fn convert(self, src: &str) -> Result<Error, Box<StdError>> {
        let (s, e) = pos_to_byte(
            src,
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
