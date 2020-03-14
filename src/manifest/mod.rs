// --- Dependencies
extern crate serde;
extern crate toml;

// --- Modules
pub mod manifest_spec;

// --- Imports
use manifest_spec::{Manifest,ManifestError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ManifestVersion<'a> {
	version: &'a str
}

/// Deserializes the contents of a manifest file based on its file type, returning a `Manifest`.
/// # Arguments
/// * `manifest` - A string slice containing the manifest file contents
pub fn deserialize_manifest(manifest: &str) -> Result<Manifest, ManifestError> {
	match manifest.find("<?xml") {
		// TOML file types
		None => match toml::from_str::<ManifestVersion>(&manifest)?.version {
			"vg-1.0" => manifest_spec::vg_1_0::deserialize_manifest(&manifest),
			_ => Err(ManifestError::UnknownType)
		},
		// XML file types
		Some(_) => manifest_spec::tq_xml::deserialize_manifest(&manifest)
	}
}