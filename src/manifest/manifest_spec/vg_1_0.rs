#[allow(non_camel_case_types)]

// --- Dependencies
extern crate serde;

// --- Imports
use serde::{Serialize, Deserialize};

/// Manifest version `vg-1.0` (Vanguard TOML 1.0)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "manifest")]
#[allow(non_camel_case_types)]
pub struct Manifest_VG_1_0 {
	/// Version identifier (ie, vg-1.0). Default is assumed to be tq-1.0 (Tequila XML spec).
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
	/// List of executable profiles.
	#[serde(rename = "profile")]
	pub profiles: Vec<MF_Profile_VG_1_0>,
	/// List of files to patch.
	#[serde(rename = "file",)]
	pub files: Vec<MF_File_VG_1_0>,
}

/// Defines a launchable application profile.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct MF_Profile_VG_1_0 {
	/// Profile name.
	pub name: String,
	/// Executable file.
	pub exec: String,
	/// Sort order of the profile for UI.
	pub order: Option<u8>,
	/// Application params for launch.
	pub params: Option<String>, 
	/// Application architecture. Assumed to be x64 if missing.
	pub architecture: Option<String>,
}

/// Defines a patchable file. MD5, SHA1, or SHA256 is required for secure patching.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct MF_File_VG_1_0 {
	/// Filepath of the file on disk, relative to app dir.
	pub path: String,
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

/// Implementation of into for Manifest generic type 
impl From<super::Manifest> for Manifest_VG_1_0 {
	/// Converts a `vg-1.0` Manifest into a generic Manifest type.
	fn from(item: super::Manifest) -> Self {
		Self {
			version     : item.version,
			label       : item.label,
			profiles    : item.profiles.into_iter().map(|e| e.into()).collect(),
			files    	: item.files.into_iter().map(|e| e.into()).collect(),
			webpage     : item.webpage,
			forums      : item.forums,
			poster_image: item.poster_image,
			discord     : item.discord,
			rss         : item.rss
		}
	}
}

/// Implementation of into for profiles
impl From<super::ManifestProfile> for MF_Profile_VG_1_0 {
	/// Converts a `vg-1.0` Profile into a generic ManifestProfile type.
	fn from(item: super::ManifestProfile) -> Self {
		Self {
			name            : item.name,
			exec            : item.exec,
			order           : item.order,
			params          : item.params,
			architecture    : item.architecture,
		}
	}
}

/// Implementation of into for files
impl From<super::ManifestFile> for MF_File_VG_1_0 {
	/// Converts a `vg-1.0` Profile into a generic ManifestFile type.
	fn from(item: super::ManifestFile) -> Self {
		Self {
			path    : item.path,
			url     : item.url,
			size    : item.size,
			md5     : item.md5,
			sha1    : item.sha1,
			sha256  : item.sha256
		}
	}
}