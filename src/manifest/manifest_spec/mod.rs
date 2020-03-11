// --- Modules
pub mod vg_1_0;
pub mod tq;

//TODO:
pub struct ManifestVersion {
	version: String,
}

/// Defines a Manifest.
/// This type is a superset of all versioned Manifest types,
/// intended to be the most generic possible typing.
pub struct Manifest {
	/// Version identifier (ie, vg-1.0).
	/// Tequila does not sepifcy a version attribute, so `tq` is used for all Tequila manifests.
	pub version: String,
	/// Global application name for the manifest.
	pub label: String,
	/// URL of the webpage for the application.
	pub webpage: Option<String>,
	/// URL of the forums for the application.
	pub forums: Option<String>,
	/// Discord invite link for the application's Discord community.
	pub discord: Option<String>,
	/// URL for an RSS news feed for the application.
	pub rss: Option<String>,
	/// URL of a banner image to display on GUI launchers.
	pub poster_image: Option<String>,
	/// List of self-updating launcher files, for Tequila only.
	pub launchers: Option<Vec<ManifestLauncher>>,
	/// List of executable profiles.
	pub profiles: Vec<ManifestProfile>,
	/// List of files to patch.
	pub files: Vec<ManifestFile>,
}

/// Defines a launchable application profile
pub struct ManifestProfile {
	/// Profile name.
	pub name: String,
	/// Executable file.
	pub exec: String,
	/// Sort order of the profile for UI.
	pub order: Option<u8>,
	/// Application params for launch.
	pub params: Option<String>,
	/// Application architecture.
	pub architecture: Option<String>,
}

/// Defines a patchable file. MD5, SHA1, or SHA256 is required for secure patching.
pub struct ManifestFile {
	/// Filepath of the file, relative to app dir.
	pub name: String,
	/// URL(s) to retrieve the file from.
	pub url: Vec<String>,
	/// Size in bytes of the file.
	pub size: Option<u64>,
	/// MD5 hash of the file.
	pub md5: Option<String>,
	/// SHA1 hash of the file.
	pub sha1: Option<String>,
	/// SHA256 hash of the file.
	pub sha256: Option<String>,
}

/// Defines the launcher for self-updating. This feature is not used by Vanguard, but is present for Tequila interoperability.
/// Launchers should only be present in Tequila XML manifests.
pub struct ManifestLauncher {
	/// Launcher id. Normally "tequila"
	pub id: String,
	/// URL(s) to retrieve the file from, in order of preference
	pub url: Vec<String>,
	/// Size in bytes of the file
	pub size: String,
	/// MD5 hash of the launcher file
	pub md5: String,
	/// Version identifier for the launcher
	pub version: String,
}