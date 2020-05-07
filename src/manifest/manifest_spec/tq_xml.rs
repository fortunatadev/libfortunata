// --- Imports
use super::{Manifest, ManifestFile, ManifestProfile};
use super::super::ManifestError;
use roxmltree::Document;

// --- Consts
pub const VERSION: &str = "tq-xml";

const TQ_TAG_MANIFEST: &str = "manifest";
const TQ_TAG_LABEL: &str = "label";
const TQ_TAG_PROFILES: &str = "profiles";
const TQ_TAG_LAUNCH: &str = "launch";
const TQ_TAG_FILELIST: &str = "filelist";
const TQ_TAG_FILE: &str = "file";
const TQ_TAG_URL: &str = "url";
const TQ_TAG_FORUMS: &str = "forums";
const TQ_TAG_FORUM: &str = "forum";
const TQ_TAG_WEBPAGE: &str = "webpage";
const TQ_TAG_POSTER: &str = "poster_image";
// --- Tags added by Fortunata, not officially supported by Tequila spec
const TQ_TAG_DISCORD: &str = "discord";
const TQ_TAG_RSS: &str = "rss";

const TQ_ATTR_EXEC: &str = "exec";
const TQ_ATTR_NAME: &str = "name";
const TQ_ATTR_PARAMS: &str = "params";
const TQ_ATTR_ICON: &str = "icon";
const TQ_ATTR_ORDER: &str = "order";
const TQ_ATTR_ARCH: &str = "architecture";
const TQ_ATTR_URL: &str = "url";
const TQ_ATTR_SIZE: &str = "size";
const TQ_ATTR_MD5: &str = "md5";
// --- Attributes added by Fortunata, not officially supported by Tequila spec
const TQ_ATTR_SHA1: &str = "sha1";
const TQ_ATTR_SHA256: &str = "sha256";

const INITIAL_PROFILE_ALLOC: usize = 127;
const INITIAL_URL_ALLOC: usize = 127;
const INITAL_FILE_ALLOC: usize = 1024;

/// Deserializes the contents of a Tequila XML (`tq-xml`) manifest file, returning a `Manifest`.
/// Note that tq_xml does not support re-serialization, but deserialized XML manifests may be output through other serializers.
/// # Arguments
/// * `manifest` - A string slice containing the manifest file contents
pub fn deserialize_manifest(manifest: &str) -> Result<Manifest, ManifestError> {
	// A Tequila manifest should only have one root element named `manifest`.
	let xml_doc = Document::parse(manifest)?;
	let xml_manifest = xml_doc.root().children().next().ok_or(ManifestError::MissingRequiredValue(TQ_TAG_MANIFEST))?;
	if !xml_manifest.is_element() || xml_manifest.tag_name().name() != TQ_TAG_MANIFEST {
		return Err(ManifestError::MissingRequiredValue(TQ_TAG_MANIFEST));
	}
	// Init base return object
	let mut manifest = Manifest {
		version: VERSION.to_owned(),
		label: "Unknown".to_owned(),
		profiles: Vec::<ManifestProfile>::with_capacity(INITIAL_PROFILE_ALLOC),
		files: Vec::<ManifestFile>::with_capacity(INITAL_FILE_ALLOC),
		webpage: None,
		forums: None,
		poster_image: None,
		discord: None,
		rss: None,
	};

	// Iterate over root children. Unknown / unused elements are silently discarded.
	for node in xml_manifest.children() {
		if node.is_element() {
			match node.tag_name().name() {
				TQ_TAG_PROFILES => parse_profiles(&node, &mut manifest)?,
				TQ_TAG_FILELIST => parse_filelist(&node, &mut manifest)?,
				TQ_TAG_FORUMS => parse_forums(&node, &mut manifest),
				TQ_TAG_LABEL => manifest.label = node.text().map(String::from).ok_or(ManifestError::MissingRequiredValue(TQ_TAG_LABEL))?,
				TQ_TAG_WEBPAGE => manifest.webpage = node.text().map(String::from),
				TQ_TAG_POSTER => manifest.poster_image = node.attribute(TQ_ATTR_URL).map(String::from),
				TQ_TAG_DISCORD => manifest.discord = node.text().map(String::from),
				TQ_TAG_RSS => manifest.rss = node.text().map(String::from),
				_ => (),
			}
		}
	}

	Ok(manifest)
}

/// Parses a Tequila XML <profiles> tree, adding profile data to the `manifest`.
fn parse_profiles(profiles: &roxmltree::Node, manifest: &mut Manifest) -> Result<(), ManifestError> {
	for node in profiles.children() {
		if node.is_element() {
			match node.tag_name().name() {
				TQ_TAG_LAUNCH => manifest.profiles.push(ManifestProfile {
					exec: node
						.attribute(TQ_ATTR_EXEC)
						.ok_or(ManifestError::MissingRequiredValue(TQ_ATTR_EXEC))?
						// Standardizes the occasional backslash in file paths
						.replace("\\", "/")
						.to_owned(),
					name: node.text().ok_or(ManifestError::MissingRequiredValue(TQ_ATTR_NAME))?.to_owned(),
					params: node.attribute(TQ_ATTR_PARAMS).map(String::from),
					icon: node.attribute(TQ_ATTR_ICON).map(String::from),
					order: node.attribute(TQ_ATTR_ORDER).and_then(|a: &str| a.parse::<u8>().ok()),
					architecture: node.attribute(TQ_ATTR_ARCH).map(String::from),
				}),
				_ => (),
			}
		}
	}
	Ok(())
}

/// Parses a Tequila XML <filelist> tree, adding profile data to the `manifest`.
fn parse_filelist(profiles: &roxmltree::Node, manifest: &mut Manifest) -> Result<(), ManifestError> {
	for node in profiles.children() {
		if node.is_element() {
			let size = node.attribute(TQ_ATTR_SIZE).and_then(|a: &str| a.parse::<u64>().ok());
			// Size = 0 is used as a delete flag in vanilla Tequila. For security reasons,
			// Fortunata will not delete a file it did not first download, so 0-size files are skipped.
			if size.is_some() && size.unwrap() == 0 {
				continue;
			}
			match node.tag_name().name() {
				TQ_TAG_FILE => {
					let mut file = ManifestFile {
						path: node
							.attribute(TQ_ATTR_NAME)
							.ok_or(ManifestError::MissingRequiredValue(TQ_ATTR_NAME))?
							// Standardizes the occasional backslash in file paths
							.replace("\\", "/")
							.to_owned(),
							mirrors: Vec::<String>::with_capacity(INITIAL_URL_ALLOC),
						size: size,
						md5: node.attribute(TQ_ATTR_MD5).map(String::from),
						sha1: node.attribute(TQ_ATTR_SHA1).map(String::from),
						sha256: node.attribute(TQ_ATTR_SHA256).map(String::from),
					};
					for url_node in node.children().filter(|n| n.tag_name().name() == TQ_TAG_URL) {
						match url_node.text() {
							Some(url) => file.mirrors.push(url.to_owned()),
							None => ()
						}
					}
					manifest.files.push(file);
				}
				_ => (),
			}
		}
	}
	Ok(())
}

/// Parses a Tequila XML <forums> tree, adding profile data to the `manifest`.
/// Fortunata does not support multiple forum URLs, so any entries after the first child are silently dropped.
fn parse_forums(forums: &roxmltree::Node, manifest: &mut Manifest) {
	manifest.forums = forums
		.children()
		.find(|n| n.tag_name().name() == TQ_TAG_FORUM)
		.and_then(|node| node.attribute(TQ_ATTR_URL).map(String::from));
}

/// Extension of ManifestError to support roxmltree error mapping
impl From<roxmltree::Error> for ManifestError {
	fn from(item: roxmltree::Error) -> Self {
		// All roxmltree parsing errors result from invalid syntax.
		ManifestError::InvalidXML(item)
	}
}

// --- Tests

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn should_deserialize() {
		let test_xml = r#"
			<manifest>
				<label>Test Manifest</label>
				<profiles>
					<launch exec="app.exe" order="0" params="--be-awesome" icon="https://example.com/icon.ico" architecture="x64">Awesome App</launch>
					<launch exec="app2.exe">Awesome App 2</launch>
				</profiles>
				<filelist>
					<file name="app.exe" size="256" md5="a-real-hash">
						<url>https://example.download.mirror/app.exe</url>
						<url>https://another.download.mirror/app.exe</url>
					</file>
					<file name="app2.exe">
						<url>https://example.download.mirror/app2.exe</url>
					</file>
					<file name="ignored.dll" size="0">
						<url>https://example.download.mirror/ignored.dll</url>
					</file>
				</filelist>
				<forums>
					<forum name="Awesome Forums" url="https://example.forums" />
				</forums>
				<webpage>https://example.com</webpage>
				<poster_image url="https://example.com/some-image.png" />
			</manifest>
		"#;
		let deser = deserialize_manifest(&test_xml).unwrap();

		assert_eq!(deser.label, "Test Manifest");

		assert_eq!(deser.profiles[0].name, "Awesome App");
		assert_eq!(deser.profiles[0].exec, "app.exe");
		assert_eq!(deser.profiles[0].order.unwrap(), 0);
		assert_eq!(deser.profiles[0].params.as_ref().unwrap(), "--be-awesome");
		assert_eq!(deser.profiles[0].icon.as_ref().unwrap(), "https://example.com/icon.ico");
		assert_eq!(deser.profiles[0].architecture.as_ref().unwrap(), "x64");

		assert_eq!(deser.profiles[1].name, "Awesome App 2");
		assert_eq!(deser.profiles[1].exec, "app2.exe");

		assert_eq!(deser.files[0].path, "app.exe");
		assert_eq!(deser.files[0].mirrors[0], "https://example.download.mirror/app.exe");
		assert_eq!(deser.files[0].mirrors[1], "https://another.download.mirror/app.exe");
		assert_eq!(deser.files[0].size.unwrap(), 256);
		assert_eq!(deser.files[0].md5.as_ref().unwrap(), "a-real-hash");

		assert_eq!(deser.files[1].path, "app2.exe");
		assert_eq!(deser.files[1].mirrors[0], "https://example.download.mirror/app2.exe");

		assert_eq!(deser.files.len(), 2);

		assert_eq!(deser.forums.unwrap(), "https://example.forums");
		assert_eq!(deser.webpage.unwrap(), "https://example.com");
		assert_eq!(deser.poster_image.unwrap(), "https://example.com/some-image.png");
	}
}
