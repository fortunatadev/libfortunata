// --- Modules
pub mod manifest_spec;

// --- Imports
use manifest_spec::Manifest;
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
			"fort-1.0" => manifest_spec::fort_1_0::deserialize_manifest(&manifest),
			_ => Err(ManifestError::UnknownType)
		},
		// XML file types
		Some(_) => manifest_spec::tq_xml::deserialize_manifest(&manifest)
	}
}

/// Serializes the contents of a `Manifest` into a a Fortunata TOML format.
/// This serialize function will output the latest `vg` manifest version.
/// Properties not supported in that format may be silently dropped.
/// To output a specific version, use a specific submodule serializer.
/// # Arguments
/// * `manifest` - The Manifest object.
pub fn serialize_manifest<'a>(manifest: &'a Manifest) -> Result<String, ManifestError> {
	// Cast to the versioned struct and overwrite the version property.
	let mut versioned_manifest: manifest_spec::fort_1_0::Manifest_VG_1_0 = manifest.into();
	versioned_manifest.version = manifest_spec::fort_1_0::VERSION;
	// Serialize
	let serialized = toml::to_string(&versioned_manifest)?;
	Ok(serialized)
}

/// Defines a Manifest IO / parse error
#[derive(Debug)]
pub enum ManifestError {
	InvalidModel(toml::ser::Error),
	InvalidSyntax(toml::de::Error),
	InvalidXML(roxmltree::Error),
	MissingRequiredValue(&'static str),
	UnknownType,
}
impl std::fmt::Display for ManifestError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			ManifestError::InvalidModel(ref e) => e.fmt(f),
			ManifestError::InvalidSyntax(ref e) => e.fmt(f),
			ManifestError::InvalidXML(ref e) => e.fmt(f),
			ManifestError::MissingRequiredValue(ref desc) => write!(f, "Missing required value: {}", desc),
			ManifestError::UnknownType => write!(f, "Could not determine manifest format/version."),
		}
	}
}
impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
			ManifestError::InvalidModel(ref e) => Some(e),
			ManifestError::InvalidSyntax(ref e) => Some(e),
			ManifestError::InvalidXML(ref e) => Some(e),
			ManifestError::MissingRequiredValue(ref _desc) => None,
			ManifestError::UnknownType => None
		}
	}
}