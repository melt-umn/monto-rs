//! The configuration for the Service.

use std::collections::BTreeSet;
use std::io::Error as IoError;
use std::net::SocketAddr;
use std::path::Path;

use url::Url;

use client::messages::ClientExtension;
use common::messages::{Identifier, SoftwareVersion};
use super::messages::ServiceExtension;

error_chain! {
    errors {
        /// A configuration file couldn't be found.
        CouldntFindConfig(path: PathBuf) {
            description("A configuration file couldn't be found")
            display("A configuration file couldn't be found at {}", path)
        }

        /// A configuration file couldn't be parsed. The error is chained onto
        /// this one.
        BadConfig(path: PathBuf) {
            description("A configuration file couldn't be parsed")
            display("The configuration file {} couldn't be parsed", path)
        }
    }

    foreign_links {
        Io(IoError, IoErrorKind)
            #[doc = "A miscellaneous I/O error when trying to load a config file."];
    }
}

/// The Service's configuration.
///
/// ## Example
///
/// ```toml
/// extensions = ["com.example.foo", "org.test.bar"]
///
/// [net]
/// addr = "[::]:28888"
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// Configuration for extensions to the Service protocol.
    #[serde(default)]
    pub extensions: BTreeSet<ServiceExtension>,

    /// Configuration for the Service's interface with Brokers.
    #[serde(default)]
    pub net: NetConfig,

    /// Configuration on how the Service should report its version and implementation.
    pub version: SoftwareVersion,
}

impl Config {
    /// Loads the configuration for a service with the given name.
    ///
    /// For a name of NAME, the following directories are searched (in order)
    /// for a file called `NAME.toml`:
    ///
    ///  - `.`
    ///  - Config home (`AppData\Roaming\NAME` / `~/Library/NAME` / `~/.config/NAME`)
    ///  - The home directory
    ///
    /// If one cannot be found, the next is used instead. If none can be found,
    /// this function returns the last error it encountered instead.
    pub fn load(name: &str) -> Result<Config, IoError> {
        use dirs::Directories;
        use std::env::home_dir;

        Config::load_one(".").or_else(|| {
            Directories::with_prefix("monto-broker", "monto-broker")
                .ok()
                .map(|dirs| dirs.config_home())
                .and_then(Config::load_one)
        }).or_else(|| {
            home_dir().and_then(Config::load_one)
        }).unwrap_or_else(|| {
            warn!("Could not open any configuration, using the default.");
            Config::default()
        })
    }

    /// Loads the config and parses command line arguments at the same time.
    ///
    /// Panics on invalid command-line arguments.
    pub fn load_with_args(name: &str, version: &str) -> Result<Config, IoError> {
        let matches = clap_app!((name) =>
            (version: version)
            (@arg CONFIG: --config +takes_value "The path to the config file.")
        ).get_matches();
        if let Some(config_path) = matches.value_of_os("CONFIG") {
            Config::load_one(&config_path)
        } else {
            Config::load()
        }
    }

    fn load_one<P: AsRef<Path>>(dir: P) -> Result<Config, IoError> {
        use std::fs::File;
        use std::io::Read;
        use toml::from_slice;

        // Build the path.
        let path = dir.as_ref().join("monto-broker.toml");

        // Open the file.
        let mut f = File::open(&path)?;

        // Create a buffer to store the file, and read the file into it.
        let mut buf = Vec::new();
        if let Err(err) = f.read_to_end(&mut buf) {
            error!("Error reading config file `{}': {}", path.display(), err);
            return None;
        }

        // Convert the file's contents to the Config type and return.
        from_slice(&buf)
    }
}

/// The configuration for how the Broker serves to Clients.
///
/// ## Example
///
/// ```toml
/// addr = "0.0.0.0:28888"
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct NetConfig {
    /// The address to serve on. Defaults to `0.0.0.0:28888`.
    pub addr: SocketAddr,
}

impl Default for NetConfig {
    fn default() -> NetConfig {
        use std::net::{Ipv4Addr, SocketAddrV4};

        let addr = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddrV4::new(addr, 28888);
        let addr = SocketAddr::V4(addr);
        NetConfig { addr }
    }
}
