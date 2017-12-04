use std::error::Error;

use monto3_common::messages::Language;
use monto3_common::products::{HighlightingColor, HighlightingToken};
use monto3_service::helpers::one_to_one_fn;
use monto3_service::messages::ServiceError;
use serde_json::{from_value, to_value};

use pos_to_byte;

simple_service_provider! {
    name = Highlighting;
    product = "highlighting";
    language = "c";
    (p, ps) => {
        one_to_one_fn(p, ps, "edu.umn.cs.melt.ablec.highlighting".parse().unwrap(), Language::C, |toks| {
            match from_value(toks) {
                Ok(toks) => {
                    let toks: Vec<MeltToken> = toks;
                    let r = toks.into_iter()
                        .map(|t| t.convert(p))
                        .collect::<Result<Vec<HighlightingToken>, _>>();
                    let r = match r {
                        Ok(x) => match to_value(x) {
                            Ok(x) => Ok(x),
                            Err(e) => Err(vec![ServiceError::Other(e.to_string())]),
                        }
                        Err(e) => Err(vec![ServiceError::Other(e.to_string())]),
                    };
                    (r, vec![])
                }
                Err(e) => (Err(vec![ServiceError::Other(e.to_string())]), vec![]),
            }
        })
    }
}

#[derive(Deserialize)]
struct MeltToken {
    color: HighlightingColor,
    start_col: usize,
    start_line: usize,
    end_col: usize,
    end_line: usize,
}

impl MeltToken {
    fn convert(self, path: &str) -> Result<HighlightingToken, Box<Error>> {
        let (s, e) = pos_to_byte(path, (self.start_line, self.start_col), (
            self.end_line,
            self.end_col,
        ))?;
        Ok(HighlightingToken {
            color: self.color,
            start_byte: s,
            end_byte: e,
        })
    }
}
