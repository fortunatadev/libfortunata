// --- Imports
use std::path::PathBuf;
use std::sync::{Arc,Mutex};
use super::config::ManifestConfig;
use super::manifest::manifest_spec::Manifest;

// --- Consts
const PARTIAL_FILE_EXT: &str = ".part";

/// Syncs application files as described in a Manifest file, downloading, deleting,
/// and modifying filesystem contents as necessary.
pub fn sync_manifest_files<'a>(manifest: &Manifest, config: &ManifestConfig) -> Result<Vec<SyncFileResult<'a>>, SyncError> {
    // Init result container
    let mut result_vec: Vec<SyncFileResult> = Vec::with_capacity(manifest.files.len());

    // Get current vanguard root dir
    let root_path = std::env::current_dir()?;
    let abs_root_path = root_path
        .into_os_string()
        .into_string()
        .or(Err(SyncError::BadPath(String::from("Could not determine the application root path."))))?;

    // Build local app path
    let mut app_path = PathBuf::new();
    app_path.push(&config.application_path);
    let abs_app_path = app_path
        .canonicalize()?
        .into_os_string()
        .into_string()
        .or(Err(SyncError::BadPath(String::from("Could not determine the Vanguard application directory."))))?;

    // Prevent syncs to the Vanguard root directory.
    if abs_root_path == abs_app_path {
        return Err(SyncError::BadPath(String::from("The Vanguard root directory cannot be used as an application root directory.")));
    }

    // Init state tracking
    // TODO:

    // TODO: Multithreaaaad

    for file in &manifest.files {
        // Build local file path
        let mut file_path = app_path.clone();
        file_path.push(&file.path);
        let abs_file_path = file_path
            .canonicalize()?
            .into_os_string()
            .into_string()
            .or(Err(SyncError::BadPath(String::from("Could not determine local file path."))))?;

        // Check path safety (inside app dir)
        if !config.allow_unsafe_file_paths && !abs_file_path.starts_with(&abs_app_path) {
            result_vec.push(SyncFileResult {
                path: file_path,
                error: Some("File path resolved to an unsafe location outside the main app directory.")
            });
            continue;
        }

        // Build a .part file path
        let mut partial_file_path = app_path.clone();
        partial_file_path.push(format!("{}{}", &file.path, PARTIAL_FILE_EXT));

        // Stream file to disk
        for mirror_url in &file.mirrors {
            // Check url for https
            if !config.allow_insecure_patching && !mirror_url.starts_with("https://") {
                continue;
            }
        }


    }

    Ok(result_vec)
}

fn check_sync_state(path: PathBuf) {

}

/// Defines the result of a file sync. Any result with a non-None `err` is considered an error response.
pub struct SyncFileResult<'a> {
    /// Absolute path to the file on the filesystem
    pub path: PathBuf,
    /// Error description. If set, result is assumed to be an error.
    pub error: Option<&'a str>
}

struct SyncTask {
    
}

/// Sync-related errors
#[derive(Debug)]
pub enum SyncError {
    BadPath(String),
    FileIO(std::io::Error),
}
impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SyncError::BadPath(ref desc) => write!(f, "Bad file path - {}", desc),
            SyncError::FileIO(ref e) => e.fmt(f),
        }
    }
}
impl std::error::Error for SyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SyncError::BadPath(ref _desc) => None,
            SyncError::FileIO(ref e) => Some(e),
        }
    }
}
impl From<std::io::Error> for SyncError {
    fn from(item: std::io::Error) -> SyncError {
        SyncError::FileIO(item)
    }
}