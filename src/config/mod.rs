// --- Imports
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs;
use std::io;

// --- Consts
/// Version identifier
pub const CFG_VERSION: &str = "1.0";
/// Config file name
pub const CFG_FILE_NAME: &str = "vanguard.toml";

/// Returns hardcoded default config
pub fn get_default_config() -> Config {
    Default::default()
}

/// Reads config entires from a `vanguard.toml` file.
pub fn read_config_file() -> Result<Config, ConfigError> {
    // Build file path
    let mut cfg_file_path = std::env::current_dir()?;
    cfg_file_path.push(CFG_FILE_NAME);
    // Read file
    let cfg_file_contents = fs::read_to_string(cfg_file_path)?;
    // Attempt toml parse
    let config = toml::de::from_str(&cfg_file_contents)?;
    Ok(config)
}

/// Writes a Config object back to disk as `vanguard.toml`.
pub fn write_config_file(config: &Config) -> Result<(), ConfigError> {
    // Build file path
    let mut cfg_file_path = std::env::current_dir()?;
    cfg_file_path.push(CFG_FILE_NAME);
    // Serialize current config
    let cfg_toml = toml::ser::to_string(config)?;
    // Write file
    fs::write(cfg_file_path, cfg_toml)?;
    Ok(())
}

// fn get_cfg_file_path() -> String {

// }

/// Config file data model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Version identifier for config file
    version: String,
    /// Default download path for new manifests. Default is app root.
    download_path: Option<String>,
    /// Maximum parallel file workers to use.
    maximum_parallel_files: u8,
    /// If true,
    use_symlinked_storage: bool,
    /// Array-table of manifests in use
    #[serde(rename = "manifest")]
    manifests: Vec<ManifestConfig>,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            version: CFG_VERSION.to_owned(),
            download_path: None,
            maximum_parallel_files: 4,
            use_symlinked_storage: true,
            manifests: Vec::new()
        }
    }
}

/// Per-manifest config data model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestConfig {
    /// URL of the manifest. Config entires without a manifest URL will be ignored.
    url: String,
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
            url: String::new(),
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
    InvalidModel(toml::ser::Error),
    InvalidSyntax(toml::de::Error)
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::FileIO(ref e) => e.fmt(f),
            ConfigError::InvalidConfig(ref desc) => write!(f, "Invalid config - {}", desc),
            ConfigError::InvalidModel(ref e) => e.fmt(f),
            ConfigError::InvalidSyntax(ref e) => e.fmt(f)
        }
    }
}
impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ConfigError::FileIO(ref e) => Some(e),
            ConfigError::InvalidConfig(ref _desc) => None,
            ConfigError::InvalidModel(ref e) => Some(e),
            ConfigError::InvalidSyntax(ref e) => Some(e),
        }
    }
}
impl From<io::Error> for ConfigError {
    fn from(item: io::Error) -> ConfigError {
        ConfigError::FileIO(item)
    }
}
impl From<toml::ser::Error> for ConfigError {
    fn from(item: toml::ser::Error) -> ConfigError {
        ConfigError::InvalidModel(item)
    }
}
impl From<toml::de::Error> for ConfigError {
    fn from(item: toml::de::Error) -> ConfigError {
        ConfigError::InvalidSyntax(item)
    }
}
