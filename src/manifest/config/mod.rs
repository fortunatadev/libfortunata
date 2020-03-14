pub struct Config {
    use serde::{Deserialize, Serialize};
    use toml::Value::Table;
    
    /// Version identifier
    const CFG_VERSION: &str = "1.0";
    const CFG_FILE_PATH: &str = "vanguard.toml";
    
    /// Loads and parses config from the `vanguard.toml` file in the application root.
    /// All errors are silently ignored, preferring to return default config.
    fn load_config() -> Config {

    }

    /// Config file data model
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Config {
        /// Array-table of manifests in use
        #[serde(rename = "manifest")]
        manifests: Option<Vec<ManifestConfig>>,
        /// Maximum parallel file workers to use.
        maximum_parallel_files: Option<u8>,
    }
    impl Default for Config {
        fn default() -> Config {
            Config {
                manifests: None,
                maximum_parallel_files: 4
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
                ignore_checksum: false,
                ignore_profiles: Vec::new()
            }
        }
    }
}