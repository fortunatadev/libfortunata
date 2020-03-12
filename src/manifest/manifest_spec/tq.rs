/// --- Dependencies
extern crate roxmltree;

// --- Imports
use super::{Manifest,ManifestProfile,ManifestFile,ManifestError};
use roxmltree::Document;

// --- Consts
const TQ_VERSION: &str		= "tq-xml";

const TQ_TAG_MANIFEST: &str = "manifest";
const TQ_TAG_PROFILES: &str = "profiles";
const TQ_TAG_LAUNCH: &str	= "launch";
const TQ_TAG_FILELIST: &str = "filelist";
const TQ_TAG_FILE: &str		= "file";
const TQ_TAG_URL: &str		= "url";
const TQ_TAG_FORUMS: &str	= "forums";
const TQ_TAG_FORUM: &str	= "forum";
const TQ_TAG_WEBPAGE: &str	= "webpage";
const TQ_TAG_POSTER: &str	= "poster_image";
// --- Tags added by Vanguard, not officially supported by Tequila spec
const TQ_TAG_DISCORD: &str	= "discord";
const TQ_TAG_RSS: &str		= "rss";

const TQ_ATTR_EXEC: &str	= "exec";
const TQ_ATTR_NAME: &str	= "name";
const TQ_ATTR_PARAMS: &str	= "params";
const TQ_ATTR_ORDER: &str	= "order";
const TQ_ATTR_ARCH: &str	= "arch";
const TQ_ATTR_URL: &str		= "url";
const TQ_ATTR_SIZE: &str	= "size";
const TQ_ATTR_MD5: &str		= "md5";
// --- Attributes added by Vanguard, not officially supported by Tequila spec
const TQ_ATTR_SHA1: &str	= "sha1";
const TQ_ATTR_SHA256: &str	= "sha256";

const INITIAL_PROFILE_ALLOC: usize	= 127;
const INITIAL_URL_ALLOC: usize		= 127;
const INITAL_FILE_ALLOC: usize		= 1024;

pub fn parse_manifest(manifest: &str) -> Result<Manifest, ManifestError> {
	// A Tequila manifest should only have one root element named `manifest`.
	let xml_doc = Document::parse(manifest)?;
	let xml_manifest = xml_doc.root().children().next().unwrap();
	if !xml_manifest.is_element() || xml_manifest.tag_name().name() != TQ_TAG_MANIFEST {
		return Err(ManifestError::Invalid);
	}

	// Init base return object
	let mut result = Manifest {
		version: TQ_VERSION.to_owned(),
		label: "None".to_owned(),
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
				TQ_TAG_PROFILES => parse_profiles(&node, &mut result),
				TQ_TAG_FILELIST => parse_filelist(&node, &mut result),
				TQ_TAG_FORUMS 	=> parse_forums(&node, &mut result),
				TQ_TAG_WEBPAGE 	=> result.webpage = node.text().map(String::from),
				TQ_TAG_POSTER 	=> result.poster_image = node.attribute(TQ_ATTR_URL).map(String::from),
				TQ_TAG_DISCORD 	=> result.discord = node.text().map(String::from),
				TQ_TAG_RSS 		=> result.rss = node.text().map(String::from),
				_ 				=> ()
			}
		}
	}

	return Ok(result);
}

/// Parses a Tequila XML <profiles> tree, adding profile data to the `manifest`.
fn parse_profiles(profiles: &roxmltree::Node, manifest: &mut Manifest) {
	for node in profiles.children() {
		if node.is_element() {
			match node.tag_name().name() {
				TQ_TAG_LAUNCH => manifest.profiles.push(ManifestProfile {
					exec: node.attribute(TQ_ATTR_EXEC).unwrap().to_owned(),
					name: node.text().unwrap().to_owned(),
					params: node.attribute(TQ_ATTR_PARAMS).map(String::from),
					order: node.attribute(TQ_ATTR_ORDER).and_then(|a: &str| a.parse::<u8>().ok()),
					architecture: node.attribute(TQ_ATTR_ARCH).map(String::from)
				}),
				_ => ()
			}
		}
	}
}

/// Parses a Tequila XML <filelist> tree, adding profile data to the `manifest`.
fn parse_filelist(profiles: &roxmltree::Node, manifest: &mut Manifest) {
	for node in profiles.children() {
		if node.is_element() {
			match node.tag_name().name() {
				TQ_TAG_FILE => {
					let mut file = ManifestFile {
						path: node.attribute(TQ_ATTR_NAME).unwrap().to_owned(),
						url: Vec::<String>::with_capacity(INITIAL_URL_ALLOC),
						size: node.attribute(TQ_ATTR_SIZE).and_then(|a: &str| a.parse::<u64>().ok()),
						md5: node.attribute(TQ_ATTR_MD5).map(String::from),
						sha1: node.attribute(TQ_ATTR_SHA1).map(String::from),
						sha256: node.attribute(TQ_ATTR_SHA256).map(String::from)
					};
					for url_node in node.children().filter(|n| n.tag_name().name() == TQ_TAG_URL) {
						file.url.push(url_node.text().unwrap().to_owned());
					}
					manifest.files.push(file);
				},
				_ => ()
			}
		}
	}
}

/// Parses a Tequila XML <forums> tree, adding profile data to the `manifest`.
/// Vanguard does not support multiple forum URLs, so any entries after the first child are silently dropped.
fn parse_forums(profiles: &roxmltree::Node, manifest: &mut Manifest) {
	manifest.forums = profiles.children().find(|n| n.tag_name().name() == TQ_TAG_FORUM).and_then(|node| node.text().map(String::from));
}