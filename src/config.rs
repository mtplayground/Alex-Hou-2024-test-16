#[cfg(feature = "ssr")]
use std::env;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppEnv {
    pub database_url: String,
    pub site_addr: SocketAddr,
    pub site_root: PathBuf,
    pub output_name: String,
}

impl AppEnv {
    #[cfg(feature = "ssr")]
    pub fn load() -> Result<Self, ConfigError> {
        load_dotenv()?;

        Ok(Self {
            database_url: required_var("DATABASE_URL")?,
            site_addr: required_var("LEPTOS_SITE_ADDR")?
                .parse()
                .map_err(|source| ConfigError::InvalidSocketAddr {
                    key: "LEPTOS_SITE_ADDR",
                    source,
                })?,
            site_root: PathBuf::from(required_var("LEPTOS_SITE_ROOT")?),
            output_name: required_var("LEPTOS_OUTPUT_NAME")?,
        })
    }
}

#[cfg(feature = "ssr")]
fn load_dotenv() -> Result<(), ConfigError> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(ConfigError::Dotenv(source)),
    }
}

#[cfg(feature = "ssr")]
fn required_var(key: &'static str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::MissingVar(key))
}

#[derive(Debug)]
pub enum ConfigError {
    MissingVar(&'static str),
    InvalidSocketAddr {
        key: &'static str,
        source: std::net::AddrParseError,
    },
    #[cfg(feature = "ssr")]
    Dotenv(dotenvy::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingVar(key) => write!(f, "missing required environment variable: {key}"),
            Self::InvalidSocketAddr { key, source } => {
                write!(f, "invalid socket address in {key}: {source}")
            }
            #[cfg(feature = "ssr")]
            Self::Dotenv(source) => write!(f, "failed to load .env file: {source}"),
        }
    }
}

impl std::error::Error for ConfigError {}
