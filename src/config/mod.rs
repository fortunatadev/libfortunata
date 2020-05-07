// --- Imports
use serde::{Deserialize, Serialize};
use std::fs;
use std::path;

// --- Consts
/// Version identifier
pub const CFG_VERSION: &str = "1.0";
/// Config file name
pub const CFG_FILE_NAME: &str = "fortunata.toml";

/// Reads config entires from a `fortunata.toml` file.
/// # Arguments
/// * `path` - Optional string slice specifying file path. Default is %fortunata-dir%/fortunata.toml.
pub fn read_config_file(path: Option<&str>) -> Result<Config, ConfigError> {
    // Read file
    let file_path = match path {
        Some(val) => path::PathBuf::from(val.to_owned()),
        None => get_default_cfg_file_path()?
    };
    let cfg_file_contents = fs::read_to_string(file_path)?;
    // Attempt toml parse
    let config = toml::de::from_str(&cfg_file_contents)?;
    Ok(config)
}

/// Writes a Config object back to disk as `fortunata.toml`.
/// /// # Arguments
/// * `config` - Config struct containing the config values to write
/// * `path` - Optional string slice specifying file path. Default is %fortunata-dir%/fortunata.toml.
pub fn write_config_file(config: &Config, path: Option<&str>) -> Result<(), ConfigError> {
    // Serialize current config
    let cfg_toml = toml::ser::to_string(config)?;
    // Write file
    let file_path = match path {
        Some(val) => path::PathBuf::from(val.to_owned()),
        None => get_default_cfg_file_path()?
    };
    fs::write(file_path, cfg_toml)?;
    Ok(())
}

/// Gets the current application directory as a PathBuf.
pub fn get_default_cfg_file_path() -> Result<path::PathBuf, ConfigError> {
    let mut cfg_file_path = std::env::current_dir()?;
    cfg_file_path.push(CFG_FILE_NAME);
    Ok(cfg_file_path)
}

/// Config file data model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Version identifier for config file
    pub version: String,
    /// Array-table of manifests in use
    #[serde(rename = "manifest")]
    pub manifests: Vec<ManifestConfig>,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            version: CFG_VERSION.to_owned(),
            manifests: Vec::new()
        }
    }
}

/// Per-manifest config data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestConfig {
    /// URL of the manifest. Config entires without a manifest URL will be ignored.
    pub url: String,
    /// If true, allows Fortunata to modify files outside the `application_path`.
    /// This is __extremely unsafe__, and may allow Fortunata to modify system files.
    pub allow_unsafe_file_paths: bool,
    /// If true, allows patching from non-https mirrors.
    pub allow_insecure_patching: bool,
    /// Path to which application files are downloaded.
    pub application_path: String,
    /// If true, Fortunata will destroy and completely rebuild the application directory on next sync.
    pub force_clean_sync: bool,
    /// If true, file consistency hash checks are not performed on files.
    pub ignore_hash_check: bool,
    /// Maximum parallel file workers to use.
    pub maximum_parallel_files: u8,
    /// If true, files are downloaded to a central location
    /// and application directories are built using symlinks
    pub use_symlinked_storage: bool,
}

/// Wrapper for config-related errors.
#[derive(Debug)]
pub enum ConfigError {
    FileIO(std::io::Error),
    InvalidConfig(String),
    InvalidModel(toml::ser::Error),
    InvalidSyntax(toml::de::Error)
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConfigError::FileIO(ref e) => e.fmt(f),
            ConfigError::InvalidConfig(ref desc) => write!(f, "Invalid config - {}", desc),
            ConfigError::InvalidModel(ref e) => e.fmt(f),
            ConfigError::InvalidSyntax(ref e) => e.fmt(f)
        }
    }
}
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfigError::FileIO(ref e) => Some(e),
            ConfigError::InvalidConfig(ref _desc) => None,
            ConfigError::InvalidModel(ref e) => Some(e),
            ConfigError::InvalidSyntax(ref e) => Some(e),
        }
    }
}
impl From<std::io::Error> for ConfigError {
    fn from(item: std::io::Error) -> ConfigError {
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
