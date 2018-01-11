extern crate aho_corasick;
extern crate either;
#[macro_use]
extern crate log;
extern crate monto3_protocol;
#[macro_use]
extern crate monto3_service;
extern crate pretty_logger;
extern crate serde_json;
extern crate tokio_core;
extern crate unicode_segmentation;
extern crate void;

mod find_todos;
mod helpers;

use either::{Left, Right};
use monto3_protocol::Language;
use monto3_protocol::products::{HighlightingColor, HighlightingToken};
use monto3_service::Service;
use monto3_service::config::Config;
use serde_json::{to_value, Value};
use tokio_core::reactor::Core;
use unicode_segmentation::UnicodeSegmentation;
use void::unreachable;

use find_todos::find_todos;
use helpers::simple_fn;

fn main() {
    pretty_logger::init_to_defaults().unwrap();
    let mut c = Core::new().unwrap();
    let config = Config::load("example_services");
    let mut s = Service::new(config, c.handle());

    s.add_provider(CharCount);
    s.add_provider(LineCount);
    s.add_provider(Reverse);
    s.add_provider(TodoFinder);

    let err = match c.run(s.serve_forever()) {
        Ok(void) => unreachable(void),
        Err(Right(err)) => err,
        Err(Left(void)) => unreachable(void),
    };
    error!("{}", err);
}

simple_service_provider! {
    name = CharCount;
    product = "edu.umn.cs.melt.monto_example_services.char_count";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| -> Result<_, &str> {
            Ok(src.len().into())
        })
    }
}

simple_service_provider! {
    name = LineCount;
    product = "edu.umn.cs.melt.monto_example_services.line_count";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| -> Result<_, &str> {
            let line_count = src.chars()
                .filter(|&c| c == '\n')
                .count();
            Ok(line_count.into())
        })
    }
}

simple_service_provider! {
    name = Reverse;
    product = "edu.umn.cs.melt.monto_example_services.reverse";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| -> Result<_, &str> {
            let mut graphemes = src.graphemes(true)
                .collect::<Vec<_>>();
            graphemes.reverse();
            let reversed = graphemes.into_iter()
                .collect::<String>();
            Ok(Value::String(reversed))
        })
    }
}

simple_service_provider! {
    name = TodoFinder;
    product = "highlighting";
    language = "text";
    (p, ps) => {
        simple_fn(p, ps, Language::Text, |src| {
            let hltoks = find_todos(&src).into_iter().map(|(s, e)| {
                HighlightingToken {
                    start_byte: s,
                    end_byte: e,
                    color: HighlightingColor::Palette(1),
                }
            }).collect::<Vec<_>>();
            to_value(hltoks)
        })
    }
}
