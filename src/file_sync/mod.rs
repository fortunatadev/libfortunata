// --- Imports
use std::path::PathBuf;
use std::sync::{Arc,Mutex};
use super::config::ManifestConfig;
use super::manifest::manifest_spec::{Manifest,ManifestFile,FileHashAlg};

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
    // TODO: this
    // TODO: needed for symlink storage?

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

        // Build the temp file path used for downloading. This is %hash%.part
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

/// Gets the current state manifest, which should be stored at a well known location.
// fn get_state_manifest(path: PathBuf) -> Manifest {
    
// }

/// Takes a ManifestFile and returns the most secure hash digest provided along with the algorithm used.
fn resolve_best_hash(file: &ManifestFile) -> Option<(&str, FileHashAlg)> {
    // TODO: Better way to destructure this
    match &file.sha256 {
        Some(val) => Some((val, FileHashAlg::sha256)),
        None => match &file.sha1 {
            Some(val) => Some((val, FileHashAlg::sha1)),
            None =>  match &file.md5 {
                Some(val) => Some((val, FileHashAlg::md5)),
                None => None
            }
        }
    }

}

/// Defines the result of a file sync. Any result with a non-None `err` is considered an error response.
pub struct SyncFileResult<'a> {
    /// Absolute path to the file on the filesystem
    pub path: PathBuf,
    /// Error description. If set, result is assumed to be an error.
    pub error: Option<&'a str>
}

/// Defines a SyncTask, which are calculated per-file by comparing sync and state Manifests
struct SyncTask {
    file: ManifestFile,
    op: SyncOp
}

/// Sync operations
enum SyncOp {
    update,
    delete
}

/// Sync-related errors
#[derive(Debug)]
pub enum SyncError {
    BadHash(String),
    BadPath(String),
    DownloadError(String),
    FileIO(std::io::Error)
}
impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SyncError::BadHash(ref desc) => write!(f, "File hash check error - {}", desc),
            SyncError::BadPath(ref desc) => write!(f, "Bad file path - {}", desc),
            SyncError::DownloadError(ref desc) => write!(f, "Download error - {}", desc),
            SyncError::FileIO(ref e) => e.fmt(f),
        }
    }
}
impl std::error::Error for SyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SyncError::BadHash(ref _desc) => None,
            SyncError::BadPath(ref _desc) => None,
            SyncError::DownloadError(ref _desc) => None,
            SyncError::FileIO(ref e) => Some(e),
        }
    }
}
impl From<std::io::Error> for SyncError {
    fn from(item: std::io::Error) -> SyncError {
        SyncError::FileIO(item)
    }
}