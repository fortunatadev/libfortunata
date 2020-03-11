/// --- Dependencies
extern crate serde;

// --- Imports
use serde::{Serialize, Deserialize};

// Manifest version `tq` (Tequila XML)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "manifest")]
#[allow(non_camel_case_types)]
pub struct Manifest_TQ {
	/// Global application name for the manifest
	pub label: String,
	/// List of executable profiles
	pub profiles: Vec<MF_Profile_TQ>,
	/// List of files to patch
	pub filelist: Vec<MF_File_TQ>,
	/// List of self-updatable files (used by Tequila, not Vanguard)
	pub launchers: Vec<MF_Launcher_TQ>,
	/// URL of the webpage for the application
	pub webpage: Option<String>,
	/// URL of the forums for the application
	pub forums: Option<String>,
	/// URL of a banner image to display on GUI launchers
	pub poster_image: Option<String>,
}

/// Defines a launchable application profile
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "launch")]
#[allow(non_camel_case_types)]
pub struct MF_Profile_TQ {
	/// Executable file
	pub exec: String,
	/// Sort order of the profile for UI
	pub order: Option<String>,
	/// Application params for launch
	pub params: Option<String>,
	/// Application architecture
	pub architecture: Option<String>,
	// /// Profile name
	#[serde(rename = "$value")]
	pub name: String,
}

/// Defines a patchable file. MD5 is required for secure patching.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "file")]
#[allow(non_camel_case_types)]
pub struct MF_File_TQ {
	/// Filepath of the file, relative to app dir.
	pub name: String,
	/// URL(s) to retrieve the file from, in order of preference.
	pub url: Vec<String>,
	/// Size in bytes of the file
	pub size: Option<String>,
	/// MD5 hash of the file
	pub md5: Option<String>,
}

/// Defines the launcher for self-updating. This feature is not used by Vanguard, but is present for Tequila interoperability.
/// MF_Launchers should only be present in Tequila manifests.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "launcher")]
#[allow(non_camel_case_types)]
pub struct MF_Launcher_TQ {
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