#[macro_use]
extern crate clap;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate monto3_client;
extern crate monto3_protocol;
extern crate pretty_logger;
extern crate tokio_core;

use std::fmt::Display;
use std::process::exit;

use clap::ArgMatches;
use itertools::Itertools;
use log::LogLevelFilter;
use tokio_core::reactor::Core;

use monto3_client::{Client, Config};
use monto3_protocol::{Language, ProductIdentifier, SoftwareVersion};

fn main() {
    // Parse CLI arguments.
    let matches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg host: -h --host +takes_value "The IP or hostname of the broker to connect to")
        (@arg port: -p --port +takes_value "The port on the broker to connect to")
        (@arg quiet: -q --quiet ... "Decreases the logging level")
        (@arg verbose: -v --verbose ... "Increases the logging level")
        (@subcommand fetch =>
            (about: "Fetches a product")
            (@arg service: +required "The service to fetch from")
            (@arg product: +required "The product to fetch")
            (@arg language: +required "The language of the Product")
            (@arg path: +required "The path of the product to fetch")
            (@arg sources: ... "The source files to send with the request")
        )
        (@subcommand list =>
            (about: "Lists the available products")
        )
    ).get_matches();

    // Start logging.
    let verbosity = 3 + matches.occurrences_of("verbose");
    let quietness = matches.occurrences_of("quiet");
    let log_level = if quietness > verbosity {
        0
    } else {
        verbosity - quietness
    };
    pretty_logger::init_level(match log_level {
        0 => LogLevelFilter::Off,
        1 => LogLevelFilter::Error,
        2 => LogLevelFilter::Warn,
        3 => LogLevelFilter::Info,
        4 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    }).unwrap();

    // Create the I/O loop.
    let mut core = Core::new().expect("Couldn't create event loop");

    // Connect to the Broker.
    let config = Config {
        host: matches.value_of("host").unwrap_or("127.0.0.1").to_string(),
        port: matches
            .value_of("port")
            .map(|s| s.parse())
            .map(must)
            .unwrap_or(28888),
        version: SoftwareVersion {
            id: "edu.umn.cs.melt.monto_rs.simple_client".parse().unwrap(),
            name: None,
            vendor: None,
            major: 0,
            minor: 1,
            patch: 0,
        },
    };
    let client_handle = core.handle();
    let client = must(core.run(Client::new(config, client_handle)));

    // Delegate to the appropriate function.
    match matches.subcommand() {
        ("fetch", Some(m)) => fetch(m, client, core),
        ("list", Some(m)) => list(m, client, core),
        _ => {
            eprintln!("{}", matches.usage());
            exit(1);
        }
    }
}

fn must<T, E: Display>(r: Result<T, E>) -> T {
    match r {
        Ok(x) => x,
        Err(err) => {
            error!("{}", err);
            exit(-2);
        }
    }
}

fn fetch(args: &ArgMatches, mut client: Client, mut core: Core) {
    // Get the arguments as strings.
    let service = args.value_of("service").unwrap();
    let product = args.value_of("product").unwrap();
    let language: Language =
        args.value_of("language").unwrap().to_string().into();
    let path = args.value_of("path").unwrap();

    // Parse the arguments.
    let service = must(
        service
            .parse()
            .map_err(|()| format!("{} is not a valid identifier", service)),
    );
    let product = must(
        product
            .parse()
            .map_err(|()| format!("{} is not a valid identifier", product)),
    );

    // Send the sources.
    for source in args.values_of("sources").unwrap_or_default() {
        info!("Sending source {}", source);
        must(core.run(client.send_file(source, language.clone())));
    }

    // Request the product.
    let pi = ProductIdentifier {
        name: product,
        language: language.to_string().into(),
        path: path.to_string(),
    };
    let p = must(core.run(client.request(&service, &pi)));

    // Print the returned value.
    println!("{}", p.value);
}

fn list(_args: &ArgMatches, client: Client, _core: Core) {
    let products = client
        .products()
        .map(|(i, d)| {
            (i.to_string(), d.language.to_string(), d.name.to_string())
        })
        .sorted()
        .into_iter()
        .group_by(|&(ref s, _, _)| s.clone());
    for (service, rest) in products.into_iter() {
        println!("{}", service);
        for (lang, rest) in
            rest.group_by(|&(_, ref l, _)| l.clone()).into_iter()
        {
            println!("\t{}", lang);
            for (_, _, product) in rest {
                println!("\t\t{}", product);
            }
        }
    }
}
