// --- Dependencies
extern crate roxmltree;

// --- Modules
pub mod vg_1_0;
pub mod tq;

//TODO:
pub struct ManifestVersion<'a> {
	version: &'a str,
}

/// Defines a Manifest.
/// This type is a superset of all versioned Manifest types.
/// All data is stored as references to data in underlying versioned serde types.
pub struct Manifest<'a> {
	/// Version identifier (ie, vg-1.0).
	/// Tequila does not sepifcy a version attribute, so `tq` is used for all Tequila manifests.
	pub version: &'a str,
	/// Global application name for the manifest.
	pub label: &'a str,
	/// URL of the webpage for the application.
	pub webpage: Option<&'a str>,
	/// URL of the forums for the application.
	pub forums: Option<&'a str>,
	/// Discord invite link for the application's Discord community.
	pub discord: Option<&'a str>,
	/// URL for an RSS news feed for the application.
	pub rss: Option<&'a str>,
	/// URL of a banner image to display on GUI launchers.
	pub poster_image: Option<&'a str>,
	/// List of executable profiles.
	pub profiles: Vec<ManifestProfile<'a>>,
	/// List of files to patch.
	pub files: Vec<ManifestFile<'a>>,
}

/// Defines a launchable application profile
pub struct ManifestProfile<'a> {
	/// Profile name.
	pub name: &'a str,
	/// Executable file.
	pub exec: &'a str,
	/// Sort order of the profile for UI.
	pub order: Option<u8>,
	/// Application params for launch.
	pub params: Option<&'a str>,
	/// Application architecture.
	pub architecture: Option<&'a str>,
}

/// Defines a patchable file. MD5, SHA1, or SHA256 is required for secure patching.
pub struct ManifestFile<'a> {
	/// Filepath of the file on disk, relative to app dir.
	pub path: &'a str,
	/// URL(s) to retrieve the file from.
	pub url: Vec<&'a str>,
	/// Size in bytes of the file.
	pub size: Option<u64>,
	/// MD5 hash of the file.
	pub md5: Option<&'a str>,
	/// SHA1 hash of the file.
	pub sha1: Option<&'a str>,
	/// SHA256 hash of the file.
	pub sha256: Option<&'a str>,
}

/// Defines a Manifest parsing error
#[derive(Debug)]
pub enum ManifestError {
	Invalid
}
impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ManifestError::Invalid => f.write_str("Invalid Manifest"),
        }
    }
}
impl std::error::Error for ManifestError {
	fn description(&self) -> &str {
        match *self {
            ManifestError::Invalid => "Failed to validated Manifest file.",
        }
    }
}
impl From<roxmltree::Error> for ManifestError {
	fn from(_: roxmltree::Error) -> Self {
		Self::Invalid
	}
}