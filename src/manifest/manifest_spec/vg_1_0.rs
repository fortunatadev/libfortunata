#[allow(non_camel_case_types)]

// --- Dependencies
extern crate serde;

// --- Imports
use serde::{Serialize, Deserialize};

/// Manifest version `vg-1.0` (Vanguard TOML 1.0)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "manifest")]
#[allow(non_camel_case_types)]
pub struct Manifest_VG_1_0<'a> {
	/// Version identifier (ie, vg-1.0). Default is assumed to be tq-1.0 (Tequila XML spec).
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
	#[serde(rename = "profile")]
	pub profiles: Vec<MF_Profile_VG_1_0<'a>>,
	/// List of files to patch.
	#[serde(rename = "file",)]
	pub files: Vec<MF_File_VG_1_0<'a>>,
}

/// Defines a launchable application profile.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct MF_Profile_VG_1_0<'a> {
	/// Profile name.
	pub name: &'a str,
	/// Executable file.
	pub exec: &'a str,
	/// Sort order of the profile for UI.
	pub order: Option<u8>,
	/// Application params for launch.
	pub params: Option<&'a str>, 
	/// Application architecture. Assumed to be x64 if missing.
	pub architecture: Option<&'a str>,
}

/// Defines a patchable file. MD5, SHA1, or SHA256 is required for secure patching.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct MF_File_VG_1_0<'a> {
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

/// Implementation of VG Manifest 1.0 -> Manifest conversion
impl From<Manifest_VG_1_0<'_>> for super::Manifest {
	fn from(item: Manifest_VG_1_0) -> Self {
		Self {
			version     : item.version.to_owned(),
			label       : item.label.to_owned(),
			profiles    : item.profiles.into_iter().map(|e| e.into()).collect(),
			files    	: item.files.into_iter().map(|e| e.into()).collect(),
			webpage     : item.webpage.map(String::from),
			forums      : item.forums.map(String::from),
			poster_image: item.poster_image.map(String::from),
			discord     : item.discord.map(String::from),
			rss         : item.rss.map(String::from)
		}
	}
}
impl From<MF_Profile_VG_1_0<'_>> for super::ManifestProfile {
	fn from(item: MF_Profile_VG_1_0) -> Self {
		Self {
			name            : item.name.to_owned(),
			exec            : item.exec.to_owned(),
			order           : item.order,
			params          : item.params.map(String::from),
			architecture    : item.architecture.map(String::from),
		}
	}
}
impl From<MF_File_VG_1_0<'_>> for super::ManifestFile {
	fn from(item: MF_File_VG_1_0) -> Self {
		Self {
			path    : item.path.to_owned(),
			url     : item.url.into_iter().map(String::from).collect(),
			size    : item.size,
			md5     : item.md5.map(String::from),
			sha1    : item.sha1.map(String::from),
			sha256  : item.sha256.map(String::from)
		}
	}
}

/// Implementation of &Manifest -> VG Manifest 1.0 conversion
impl<'a> From<&'a super::Manifest> for Manifest_VG_1_0<'a> {
	fn from(item: &'a super::Manifest) -> Self {
		Self {
			version     : item.version.as_str(),
			label       : item.label.as_str(),
			profiles    : item.profiles.iter().map(|e| MF_Profile_VG_1_0::<'a> {
				name: e.name.as_str(),
				exec: e.exec.as_str(),
				order: e.order,
				params: e.params.as_deref(),
				architecture: e.architecture.as_deref()
			}).collect(),
			files	    : item.files.iter().map(|e| MF_File_VG_1_0::<'a> {
				path: e.path.as_str(),
				url: e.url.iter().map(|u| u.as_str()).collect(),
				size: e.size,
				md5: e.md5.as_deref(),
				sha1: e.sha1.as_deref(),
				sha256: e.sha256.as_deref(),
			}).collect(),
			webpage     : item.webpage.as_deref(),
			forums      : item.forums.as_deref(),
			poster_image: item.poster_image.as_deref(),
			discord     : item.discord.as_deref(),
			rss         : item.rss.as_deref()
		}
	}
}