//! The configuration for the Broker.

use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::path::Path;

use url::Url;

use monto3_client::messages::ClientExtension;
use monto3_common::messages::{Identifier, SoftwareVersion};
use monto3_service::messages::ServiceExtension;

/// The Broker's configuration.
///
/// ## Example
///
/// ```toml
/// [[service]]
/// addr = "monto.example.org"
///
/// [[service]]
/// addr = "localhost:12345"
/// base = "/ableC/monto"
///
/// [[service]]
/// addr = "localhost:12345"
/// base = "/silver/monto"
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Configuration for the Broker's own behavior. This controls a few behaviors the
    /// specification intentionally leaves up to implementations.
    pub broker: BrokerConfig,

    /// Configuration for extensions to the Monto protocols.
    pub extensions: ExtensionConfig,

    /// Configuration for the Broker's interface with Clients.
    pub net: NetConfig,

    /// Configuration for the services to connect to.
    pub service: Vec<ServiceConfig>,

    /// Configuration on how the Broker should report its version and implementation.
    pub version: VersionConfig,
}

impl Config {
    /// Loads the configuration.
    ///
    /// The following directories are searched (in order) for a file called `monto-broker.toml`:
    ///
    ///  - `.`
    ///  - Config home (`AppData\Roaming\monto-broker` / `~/Library/monto-broker` / `~/.config/monto-broker`)
    ///  - The home directory
    ///
    /// If one cannot be found, the next is used instead. If none can be found, the default
    /// configuration is used.
    ///
    /// Note that the default configuration does not connect to any services. As
    /// such, a warning will be emitted if it is used.
    pub fn load() -> Config {
        use dirs::Directories;
        use std::env::home_dir;

        Config::load_one(".")
            .or_else(|| {
                Directories::with_prefix("monto-broker", "monto-broker")
                    .ok()
                    .map(|dirs| dirs.config_home())
                    .and_then(Config::load_one)
            })
            .or_else(|| home_dir().and_then(Config::load_one))
            .unwrap_or_else(|| {
                warn!("Could not open any configuration, using the default.");
                Config::default()
            })
    }

    /// Loads the config and parses command line arguments at the same time.
    ///
    /// Panics on invalid command-line arguments.
    pub fn load_with_args(name: &str, version: &str) -> Config {
        let matches = clap_app!((name) =>
            (version: version)
            (@arg CONFIG: --config +takes_value "The path to the config file.")
        ).get_matches();
        if let Some(config_path) = matches.value_of_os("CONFIG") {
            Config::load_one(&config_path).unwrap_or_else(|| panic!("Failed to load config."))
        } else {
            Config::load()
        }
    }

    fn load_one<P: AsRef<Path>>(dir: P) -> Option<Config> {
        use std::fs::File;
        use std::io::{ErrorKind, Read};
        use toml::from_slice;

        // Build the path.
        let path = dir.as_ref().join("monto-broker.toml");

        // Open the file.
        let mut f = match File::open(&path) {
            Ok(f) => f,
            Err(err) => {
                if err.kind() != ErrorKind::NotFound {
                    error!("Error opening config file `{}': {}", path.display(), err);
                }
                return None;
            }
        };

        // Create a buffer to store the file, and read the file into it.
        let mut buf = Vec::new();
        if let Err(err) = f.read_to_end(&mut buf) {
            error!("Error reading config file `{}': {}", path.display(), err);
            return None;
        }

        // Convert the file's contents to the Config type and return.
        match from_slice(&buf) {
            Ok(config) => Some(config),
            Err(err) => {
                error!("Error parsing config file `{}': {}", path.display(), err);
                None
            }
        }
    }
}

/// The configuration for implementation-specific behavior.
///
/// ## Example
/// ```toml
/// service_failure_is_fatal = true
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BrokerConfig {
    /// Whether to treat failure to connect to a Service during startup as fatal.
    ///
    /// Defaults to true.
    pub service_failure_is_fatal: bool,
}

impl Default for BrokerConfig {
    fn default() -> BrokerConfig {
        BrokerConfig {
            service_failure_is_fatal: true,
        }
    }
}

/// The configuration for extensions.
///
/// ## Example
///
/// ```toml
/// client = ["com.example.foo"]
/// service = []
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExtensionConfig {
    /// Client Protocol Extensions that are available.
    pub client: BTreeSet<ClientExtension>,

    /// Service Protocol Extensions that are available.
    pub service: BTreeSet<ServiceExtension>,
}

/// The configuration for how the Broker serves to Clients.
///
/// ## Example
///
/// ```toml
/// TODO
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

/// The configuration for a Broker to connect to a Service.
///
/// ## Example
///
/// ```toml
/// addr = "localhost:1234"
/// base = "/monto"
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    /// The address the service is at.
    pub addr: String,

    /// The base part of the Monto Service URI. Defaults to `/monto`.
    #[serde(default = "ServiceConfig::default_base")]
    pub base: String,

    /// The URI Scheme to use. Defaults to "http".
    #[serde(default = "ServiceConfig::default_scheme")]
    pub scheme: String,
}

impl ServiceConfig {
    /// Builds a URL from the URL parts.
    pub fn build_url<K, P, PI, QI, V>(&self, path: PI, query: QI) -> Url
    where
        P: AsRef<str>,
        PI: IntoIterator<Item = P>,
        K: AsRef<str>,
        V: AsRef<str>,
        QI: IntoIterator<Item = (K, V)>,
    {
        // Build the base URL.
        let url = format!("{}://{}{}", self.scheme, self.addr, self.base);
        let mut url = Url::parse(&url).unwrap();

        // Append the path parts to the URL.
        url.path_segments_mut()
            .expect("ServiceConfig-built URL is cannot-be-a-base?")
            .extend(path);

        // Build the query part of the URL.
        for part in query {
            let (k, v) = part;
            url.query_pairs_mut().append_pair(k.as_ref(), v.as_ref());
        }

        // Return the URL.
        url
    }

    fn default_base() -> String {
        "/monto".to_string()
    }
    fn default_scheme() -> String {
        "http".to_string()
    }
}

/// The configuration for a Broker's reported version.
///
/// ## Example
///
/// ```toml
/// id = "com.example.broker"
/// name = "Example Broker"
/// vendor = "ACME, Inc."
/// major = 1
/// minor = 0
/// patch = 0
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VersionConfig {
    /// The ID of the Broker.
    ///
    /// Defaults to "edu.umn.cs.melt.monto_rs.broker".
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
        VersionConfig {
            id: "edu.umn.cs.melt.monto.broker".parse().unwrap(),
            name: "Reference Implementation Broker".to_owned(),
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
