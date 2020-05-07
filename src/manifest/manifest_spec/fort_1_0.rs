// --- Imports
use super::{Manifest, ManifestFile, ManifestProfile};
use super::super::ManifestError;
use serde::{Deserialize, Serialize};

/// Version identifier
pub const VERSION: &str = "fort-1.0";

/// Serializes the contents of a `Manifest` into a Fortunata 1.0 (`fort-1.0`) TOML format.
/// Note that properties not supported in `fort-1.0` will be silently dropped.
/// # Arguments
/// * `manifest` - The Manifest object.
pub fn serialize_manifest<'a>(manifest: &'a Manifest) -> Result<String, ManifestError> {
	// Cast to the versioned struct and overwrite the version property.
	let mut versioned_manifest: Manifest_VG_1_0 = manifest.into();
	versioned_manifest.version = VERSION;
	// Serialize
	let serialized = toml::to_string(&versioned_manifest)?;
	Ok(serialized)
}

/// Deserializes the contents of a Fortunata 1.0 (`fort-1.0`) manifest file, returning a `Manifest`.
/// # Arguments
/// * `manifest` - A string slice containing the manifest file contents
pub fn deserialize_manifest(manifest: &str) -> Result<Manifest, ManifestError> {
	println!("{}", manifest);
	let versioned_manifest: Manifest_VG_1_0 = toml::from_str(manifest).unwrap();
	Ok(versioned_manifest.into())
}

/// Manifest version `fort-1.0` (Fortunata TOML 1.0)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "manifest")]
#[allow(non_camel_case_types)]
pub struct Manifest_VG_1_0<'a> {
	/// Version identifier (ie, fort-1.0).
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
	#[serde(rename = "file")]
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
	/// Application icon
	pub icon: Option<&'a str>,
	/// Application architecture. Assumed to be x64 if missing.
	pub architecture: Option<&'a str>,
}

/// Defines a patchable file. MD5, SHA1, or sha256 is required for secure patching.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct MF_File_VG_1_0<'a> {
	/// Filepath of the file on disk, relative to app dir.
	pub path: &'a str,
	/// URL(s) to retrieve the file from.
	pub mirrors: Vec<&'a str>,
	/// Size in bytes of the file.
	pub size: Option<u64>,
	/// MD5 hash of the file.
	pub md5: Option<&'a str>,
	/// SHA1 hash of the file.
	pub sha1: Option<&'a str>,
	/// sha256 hash of the file.
	pub sha256: Option<&'a str>,
}

/// Implementation of VG Manifest 1.0 -> Manifest conversion
impl From<Manifest_VG_1_0<'_>> for Manifest {
	fn from(item: Manifest_VG_1_0) -> Self {
		Self {
			version: item.version.to_owned(),
			label: item.label.to_owned(),
			profiles: item.profiles.into_iter().map(|e| e.into()).collect(),
			files: item.files.into_iter().map(|e| e.into()).collect(),
			webpage: item.webpage.map(String::from),
			forums: item.forums.map(String::from),
			poster_image: item.poster_image.map(String::from),
			discord: item.discord.map(String::from),
			rss: item.rss.map(String::from),
		}
	}
}
impl From<MF_Profile_VG_1_0<'_>> for ManifestProfile {
	fn from(item: MF_Profile_VG_1_0) -> Self {
		Self {
			name: item.name.to_owned(),
			exec: item.exec.to_owned(),
			order: item.order,
			params: item.params.map(String::from),
			icon: item.icon.map(String::from),
			architecture: item.architecture.map(String::from),
		}
	}
}
impl From<MF_File_VG_1_0<'_>> for ManifestFile {
	fn from(item: MF_File_VG_1_0) -> Self {
		Self {
			path: item.path.to_owned(),
			mirrors: item.mirrors.into_iter().map(String::from).collect(),
			size: item.size,
			md5: item.md5.map(String::from),
			sha1: item.sha1.map(String::from),
			sha256: item.sha256.map(String::from),
		}
	}
}

/// Implementation of &Manifest -> VG Manifest 1.0 conversion
impl<'a> From<&'a Manifest> for Manifest_VG_1_0<'a> {
	fn from(item: &'a Manifest) -> Self {
		Self {
			version: item.version.as_str(),
			label: item.label.as_str(),
			profiles: item
				.profiles
				.iter()
				.map(|e| MF_Profile_VG_1_0::<'a> {
					name: e.name.as_str(),
					exec: e.exec.as_str(),
					order: e.order,
					params: e.params.as_deref(),
					icon: e.icon.as_deref(),
					architecture: e.architecture.as_deref(),
				})
				.collect(),
			files: item
				.files
				.iter()
				.map(|e| MF_File_VG_1_0::<'a> {
					path: e.path.as_str(),
					mirrors: e.mirrors.iter().map(|u| u.as_str()).collect(),
					size: e.size,
					md5: e.md5.as_deref(),
					sha1: e.sha1.as_deref(),
					sha256: e.sha256.as_deref(),
				})
				.collect(),
			webpage: item.webpage.as_deref(),
			forums: item.forums.as_deref(),
			poster_image: item.poster_image.as_deref(),
			discord: item.discord.as_deref(),
			rss: item.rss.as_deref(),
		}
	}
}

/// Extension of ManifestError to support toml serializer error mapping
impl From<toml::ser::Error> for ManifestError {
	fn from(item: toml::ser::Error) -> Self {
		// All toml parsing errors result from invalid syntax.
		ManifestError::InvalidModel(item)
	}
}

/// Extension of ManifestError to support toml deserializer error mapping
impl From<toml::de::Error> for ManifestError {
	fn from(item: toml::de::Error) -> Self {
		// All toml parsing errors result from invalid syntax.
		ManifestError::InvalidSyntax(item)
	}
}

// --- Tests

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn should_serialize() {
		let test_manifest = Manifest {
			version: "fort-1.0".to_owned(),
			label: "Test Manifest".to_owned(),
			profiles: vec![ManifestProfile {
				name: "Awesome App".to_owned(),
				exec: "app.exe".to_owned(),
				params: Some("--be-awesome".to_owned()),
				icon: Some("https://example.com/icon.ico".to_owned()),
				order: Some(0),
				architecture: Some("x64".to_owned()),
			},
			ManifestProfile {
				name: "Awesome App 2".to_owned(),
				exec: "app2.exe".to_owned(),
				params: None,
				icon: None,
				order: None,
				architecture: None
			}],
			files: vec![ManifestFile {
				path: "app.exe".to_owned(),
				mirrors: vec![
					"https://example.download.mirror/app.exe".to_owned(),
					"https://another.download.mirror/app.exe".to_owned(),
				],
				size: Some(256),
				md5: Some("a-real-hash".to_owned()),
				sha1: Some("a-realer-hash".to_owned()),
				sha256: Some("the-realest-hash".to_owned()),
			},
			ManifestFile {
				path: "app2.exe".to_owned(),
				mirrors: vec![
					"https://example.download.mirror/app2.exe".to_owned(),
				],
				size: None,
				md5: None,
				sha1: None,
				sha256: None,
			}],
			webpage: Some("https://example.com".to_owned()),
			forums: Some("https://example.forums".to_owned()),
			discord: Some("https://a.discord.invite".to_owned()),
			rss: Some("https://example.com/some-rss-feed.rss".to_owned()),
			poster_image: Some("https://example.com/some-image.png".to_owned()),
		};
		let ser = serialize_manifest(&test_manifest).unwrap();

		assert_eq!(ser, r#"
			version = "fort-1.0"
			label = "Test Manifest"
			webpage = "https://example.com"
			forums = "https://example.forums"
			discord = "https://a.discord.invite"
			rss = "https://example.com/some-rss-feed.rss"
			poster_image = "https://example.com/some-image.png"

			[[profile]]
			name = "Awesome App"
			exec = "app.exe"
			order = 0
			params = "--be-awesome"
			icon = "https://example.com/icon.ico"
			architecture = "x64"

			[[profile]]
			name = "Awesome App 2"
			exec = "app2.exe"

			[[file]]
			path = "app.exe"
			mirrors = ["https://example.download.mirror/app.exe", "https://another.download.mirror/app.exe"]
			size = 256
			md5 = "a-real-hash"
			sha1 = "a-realer-hash"
			sha256 = "the-realest-hash"

			[[file]]
			path = "app2.exe"
			mirrors = ["https://example.download.mirror/app2.exe"]
		"#.replace("\t", "").replacen("\n", "", 1));
	}

	#[test]
	fn should_deserialize() {
		let test_toml = r#"
			version = "fort-1.0"
			label = "Test Manifest"
			webpage = "https://example.com"
			forums = "https://example.forums"
			discord = "https://a.discord.invite"
			rss = "https://example.com/some-rss-feed.rss"
			poster_image = "https://example.com/some-image.png"
	
			[[profile]]
			name = "Awesome App"
			exec = "app.exe"
			order = 0
			params = "--be-awesome"
			icon = "https://example.com/icon.ico"
			architecture = "x64"
	
			[[profile]]
			name = "Awesome App 2"
			exec = "app2.exe"
	
			[[file]]
			path = "app.exe"
			mirrors = ["https://example.download.mirror/app.exe", "https://another.download.mirror/app.exe"]
			size = 256
			md5 = "a-real-hash"
			sha1 = "a-realer-hash"
			sha256 = "the-realest-hash"
	
			[[file]]
			path = "app2.exe"
			mirrors = ["https://example.download.mirror/app2.exe"]
		"#;
		let deser = deserialize_manifest(&test_toml).unwrap();
	
		assert_eq!(deser.label, "Test Manifest");
	
		assert_eq!(deser.profiles[0].name, "Awesome App");
		assert_eq!(deser.profiles[0].exec, "app.exe");
		assert_eq!(deser.profiles[0].order.unwrap(), 0);
		assert_eq!(deser.profiles[0].params.as_ref().unwrap(), "--be-awesome");
		assert_eq!(deser.profiles[0].architecture.as_ref().unwrap(), "x64");
	
		assert_eq!(deser.profiles[1].name, "Awesome App 2");
		assert_eq!(deser.profiles[1].exec, "app2.exe");
	
		assert_eq!(deser.files[0].path, "app.exe");
		assert_eq!(deser.files[0].mirrors[0], "https://example.download.mirror/app.exe");
		assert_eq!(deser.files[0].mirrors[1], "https://another.download.mirror/app.exe");
		assert_eq!(deser.files[0].size.unwrap(), 256);
		assert_eq!(deser.files[0].md5.as_ref().unwrap(), "a-real-hash");
		assert_eq!(deser.files[0].sha1.as_ref().unwrap(), "a-realer-hash");
		assert_eq!(deser.files[0].sha256.as_ref().unwrap(), "the-realest-hash");
	
		assert_eq!(deser.files[1].path, "app2.exe");
		assert_eq!(deser.files[1].mirrors[0], "https://example.download.mirror/app2.exe");
	
		assert_eq!(deser.forums.unwrap(), "https://example.forums");
		assert_eq!(deser.webpage.unwrap(), "https://example.com");
		assert_eq!(deser.discord.unwrap(), "https://a.discord.invite");
		assert_eq!(deser.rss.unwrap(), "https://example.com/some-rss-feed.rss");
		assert_eq!(deser.poster_image.unwrap(), "https://example.com/some-image.png");
	}
}
