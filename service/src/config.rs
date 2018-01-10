//! The configuration for the Service.

use std::collections::BTreeSet;
use std::io::Error as IoError;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use rand::random;

use monto3_common::messages::{Identifier, SoftwareVersion};

use messages::ServiceExtension;

error_chain! {
    errors {
        /// A configuration file couldn't be found.
        CouldntFindConfig(path: PathBuf) {
            description("A configuration file couldn't be found")
            display("A configuration file couldn't be found at {}", path.display())
        }

        /// A configuration file couldn't be parsed. The error is chained onto
        /// this one.
        BadConfig(path: PathBuf) {
            description("A configuration file couldn't be parsed")
            display("The configuration file {} couldn't be parsed", path.display())
        }
    }

    foreign_links {
        Io(IoError)
            #[doc = "An I/O error when trying to load a config file."];
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Config {
    /// Configuration for extensions to the Service protocol.
    #[serde(default)]
    pub extensions: BTreeSet<ServiceExtension>,

    /// Configuration for the Service's interface with Brokers.
    #[serde(default)]
    pub net: NetConfig,

    /// Configuration on how the Service should report its version and implementation.
    #[serde(default)]
    pub version: VersionConfig,
}

impl Config {
    /// Loads the configuration for a service with the given name.
    ///
    /// For a name of NAME, the following directories are searched (in order)
    /// for a file called `NAME.toml`:
    ///
    ///  - `.`
    ///  - Config home (`AppData\Roaming\NAME`, `~/Library/NAME`, or `~/.config/NAME`)
    ///  - The home directory
    ///
    /// If one cannot be found, the next is used instead. If none can be found,
    /// this function returns the last error it encountered instead.
    pub fn load(name: &str) -> Config {
        use dirs::Directories;
        use std::env::home_dir;

        match Config::load_one(".", name) {
            Ok(c) => return c,
            Err(e) => {
                error!("Failed to load config from current directory: {}", e);
                match Directories::with_prefix(name, name) {
                    Ok(dirs) => {
                        match Config::load_one(dirs.config_home(), name) {
                            Ok(c) => return c,
                            Err(e) => {
                                error!("Failed to load config from config directory: {}", e);
                                match home_dir() {
                                    Some(home_dir) => {
                                        match Config::load_one(home_dir, name) {
                                            Ok(c) => return c,
                                            Err(e) => {
                                                error!("Failed to load config from home directory: {}", e);
                                            }
                                        }
                                    }
                                    None => error!(
                                        "Could not identify home directory"
                                    ),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to find config directory: {}", e);
                    }
                }
                warn!("Couldn't find config, loading defaults.");
                Config::default()
            }
        }
    }

    /// Loads the config and parses command line arguments at the same time.
    ///
    /// Panics on invalid command-line arguments.
    pub fn load_with_args(name: &str, version: &str) -> Result<Config> {
        let matches = clap_app!((name) =>
            (version: version)
            (@arg CONFIG: --config +takes_value "The path to the config file.")
        ).get_matches();
        if let Some(config_path) = matches.value_of_os("CONFIG") {
            Config::load_one(&config_path, name)
        } else {
            Ok(Config::load(name))
        }
    }

    fn load_one<P: AsRef<Path>>(dir: P, name: &str) -> Result<Config> {
        use std::fs::File;
        use std::io::Read;
        use toml::from_slice;

        // Build the path.
        let mut path = dir.as_ref().join(name);
        path.set_extension("toml");

        // Open the file.
        let mut f = File::open(&path)
            .chain_err(|| ErrorKind::CouldntFindConfig(path.clone()))?;

        // Create a buffer to store the file, and read the file into it.
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        // Convert the file's contents to the Config type and return.
        from_slice(&buf).chain_err(|| ErrorKind::BadConfig(path))
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            extensions: BTreeSet::new(),
            net: NetConfig::default(),
            version: VersionConfig::default(),
        }
    }
}

/// The configuration for how the Broker serves to Clients.
///
/// ## Example
///
/// ```toml
/// addr = "0.0.0.0:28888"
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

#[test]
fn netconfig() {
    extern crate toml;

    let example = r#"addr = "0.0.0.0:28888""#;
    let nc: NetConfig = toml::from_str(example).unwrap();
    assert_eq!(nc, NetConfig::default());
}

/// The configuration for a Service's reported version.
///
/// ## Example
///
/// ```toml
/// id = "com.example.service"
/// name = "Example Service"
/// vendor = "ACME, Inc."
/// major = 1
/// minor = 0
/// patch = 0
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct VersionConfig {
    /// The ID of the Broker.
    ///
    /// Defaults to "edu.umn.cs.melt.monto.servicelibxxxxxxxx", where each x is
    /// replaced by a random hex digit.
    pub id: Identifier,

    /// The name of the Broker.
    ///
    /// Defaults to "Reference Implementation Broker".
    pub name: String,

    /// The vendor of the Broker.
    ///
    /// Defaults to "Minnesota Extensible Language Tools".
    pub vendor: String,

    /// The major version of the Broker.
    ///
    /// Defaults to 0.
    pub major: u64,

    /// The minor version of the Broker.
    ///
    /// Defaults to 0.
    pub minor: u64,

    /// The patch version of the Broker.
    ///
    /// Defaults to 0.
    pub patch: u64,
}

impl Default for VersionConfig {
    fn default() -> VersionConfig {
        let random: u32 = random();
        VersionConfig {
            id: format!("edu.umn.cs.melt.monto.servicelib{:08x}", random)
                .parse()
                .unwrap(),
            name: "Reference Implementation Service Library".to_owned(),
            vendor: "Minnesota Extensible Language Tools".to_owned(),
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

impl From<VersionConfig> for SoftwareVersion {
    fn from(config: VersionConfig) -> SoftwareVersion {
        SoftwareVersion {
            id: config.id,
            name: Some(config.name),
            vendor: Some(config.vendor),
            major: config.major,
            minor: config.minor,
            patch: config.patch,
        }
    }
}
