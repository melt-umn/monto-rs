use std::env;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

// TODO: Fill in all the assigned codes.
const CODES: &[(usize, &'static str)] = &[
    (200, "Ok"),
    (204, "NoContent"),
    (400, "BadRequest"),
    (409, "Conflict"),
    (500, "InternalServerError"),
    (502, "BadGateway"),
    (503, "ServiceUnavailable"),
];

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    File::create(Path::new(&out_dir).join("status_types.rs"))
        .and_then(make_status_types)?;
    File::create(Path::new(&out_dir).join("deserialize_body.rs"))
        .and_then(make_deserialize_body)?;
    Ok(())
}

fn make_status_types(mut f: File) -> Result<()> {
    for &(code, hyper_name) in CODES.iter() {
        writeln!(
            f,
            "/// A struct that represents an HTTP status code of {} at the type level.", code).unwrap();
        writeln!(f, "pub enum Status{} {{}}\n", code).unwrap();
        writeln!(f, "impl StatusCode for Status{} {{", code).unwrap();
        writeln!(
            f,
            "\tconst VALUE: ::hyper::StatusCode = ::hyper::StatusCode::{};\n",
            hyper_name
        )?;
        writeln!(f, "}}\n")?;
    }
    Ok(())
}

fn make_deserialize_body(mut f: File) -> Result<()> {
    writeln!(f, "match status {{")?;
    for &(code, hyper_name) in CODES.iter() {
        let hyper_value = format!("::hyper::StatusCode::{}", hyper_name);
        let status_type =
            format!("::haskellism::response_map::status_types::Status{}", code);

        writeln!(
            f,
            "{} => respmap_to_aunion::<{}, _, _, _, _, _, _>(s),",
            hyper_value, status_type,
        )?;
    }
    writeln!(f, "\ts => Err(RespError::BadStatus(s)),")?;
    writeln!(f, "}}")
}
