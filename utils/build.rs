use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    make_status_types(Path::new(&out_dir).join("status_types.rs"));
}

fn make_status_types(path: PathBuf) {
    let codes = [
        (200, "Ok"),
        (204, "NoContent"),
        (400, "BadRequest"),
        (409, "Conflict"),
        (500, "InternalServerError"),
        (502, "BadGateway"),
        (503, "ServiceUnavailable"),
    ];

    let mut f = File::create(path).unwrap();
    for &(code, hyper_name) in codes.iter() {
        writeln!(
            f,
            "/// A struct that represents an HTTP status code of {} at the type level.", code).unwrap();
        writeln!(f, "pub enum Status{} {{}}\n", code).unwrap();
        writeln!(f, "impl StatusCode for Status{} {{", code).unwrap();
        writeln!(
            f,
            "\tconst VALUE: ::hyper::StatusCode = ::hyper::StatusCode::{};\n",
            hyper_name
        ).unwrap();
        writeln!(f, "}}\n").unwrap();
    }
}
