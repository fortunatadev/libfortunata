// --- Imports
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path;

// --- Consts
/// Version identifier
const CFG_VERSION: &str = "1.0";
/// Config file name
const CFG_FILE_NAME: &str = "vanguard.toml";

/// Loads and parses config from the `vanguard.toml` file in the application root.
/// All errors are silently ignored, preferring to return default config.
pub fn load_config() -> Result<Config, ConfigError> {
    let mut cfg_file_path = std::env::current_dir()?;
    Ok(Default::default())
}


/// Config file data model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default download path for new manifests. Default is app root.
    download_path: Option<String>,
    /// Maximum parallel file workers to use.
    maximum_parallel_files: u8,
    /// If true,
    use_symlinked_storage: bool,
    /// Array-table of manifests in use
    #[serde(rename = "manifest")]
    manifests: Option<Vec<ManifestConfig>>,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            download_path: None,
            maximum_parallel_files: 4,
            use_symlinked_storage: true,
            manifests: None
        }
    }
}

/// Per-manifest config data model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestConfig {
    /// URL of the manifest. Config entires without a manifest URL will be ignored.
    url: Option<String>,
    /// If true, allows patching from non-https mirrors.
    allow_insecure_patching: bool,
    /// Download path for the specific manifest. Overrides global setting. Default is app root.
    download_path: Option<String>,
    /// If true, checksumming is not performed on files.
    ignore_checksum: bool,
    /// Vec of launcher profiles to hide, by name.
    ignore_profiles: Vec<String>,
}
impl Default for ManifestConfig {
    fn default() -> ManifestConfig {
        ManifestConfig {
            url: None,
            allow_insecure_patching: false,
            download_path: None,
            ignore_checksum: false,
            ignore_profiles: Vec::new()
        }
    }
}

/// Wrapper for config-related errors.
#[derive(Debug)]
pub enum ConfigError {
    FileIO(io::Error),
    InvalidConfig(String),
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::FileIO(ref e) => e.fmt(f),
            ConfigError::InvalidConfig(ref desc) => write!(f, "Invalid config - {}", desc)
        }
    }
}
impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ConfigError::FileIO(ref e) => Some(e),
            ConfigError::InvalidConfig(ref _desc) => None
        }
    }
}
impl From<io::Error> for ConfigError {
    fn from(item: io::Error) -> ConfigError {
        ConfigError::FileIO(item)
    }
}