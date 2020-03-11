/// --- Dependencies
extern crate roxmltree;

// --- Imports
use super::{Manifest,ManifestProfile,ManifestFile,ManifestError};
use roxmltree::Document;

// --- Consts
const TQ_VERSION: &str		= "tq";

const TQ_TAG_MANIFEST: &str = "manifest";
const TQ_TAG_PROFILES: &str = "profiles";
const TQ_TAG_LAUNCH: &str	= "launch";
const TQ_TAG_FILELIST: &str = "filelist";
const TQ_TAG_FILE: &str		= "file";

const TQ_ATTR_EXEC: &str	= "exec";
const TQ_ATTR_NAME: &str	= "name";
const TQ_ATTR_PARAMS: &str	= "params";
const TQ_ATTR_ORDER: &str	= "order";
const TQ_ATTR_ARCH: &str	= "arch";

const INITIAL_PROFILE_ALLOC: usize	= 127;
const INITAL_FILE_ALLOC: usize		= 1024;

pub fn parse_manifest<'a>(manifest: &str) -> Result<Manifest, ManifestError> {
	// A Tequila manifest should only have one root element named `manifest`.
	let xml_doc = Document::parse(manifest)?;
	let xml_manifest = xml_doc.root().children().next().unwrap();
	if !xml_manifest.is_element() || xml_manifest.tag_name().name() != TQ_TAG_MANIFEST {
		return Err(ManifestError::Invalid);
	}

	// Build the start of our return object.
	let mut result = Manifest::<'a> {
		version: TQ_VERSION,
		label: "None",
		profiles: Vec::<ManifestProfile>::with_capacity(INITIAL_PROFILE_ALLOC),
		files: Vec::<ManifestFile>::with_capacity(INITAL_FILE_ALLOC),
		webpage: None,
		forums: None,
		poster_image: None,
		discord: None,
		rss: None,
	};

	// Iterate over root children. Non-matching elements are silently discarded.
	for node in xml_manifest.children() {
		if node.is_element() {
			println!("{:?}", node);
			match node.tag_name().name() {
				TQ_TAG_PROFILES => parse_profiles(&node, &mut result),
				_ => ()
			}
		}
	}

	return Ok(super::Manifest {
		version: "vg-1.0",
		label: "Example Manifest",
		profiles: vec![super::ManifestProfile {
			name: "Example App",
			exec: "example-app.exe",
			params: Some("--be-super-awesome"),
			order: Some(0),
			architecture: Some("x64")
		}],
		files: vec![super::ManifestFile {
			path: "example-app.exe",
			url: vec!["https://some.example.app/example-app.exe"],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-totally-a-real-hash")
		},
		super::ManifestFile {
			path: "example-dep.dll",
			url: vec!["https://some.example.app/example-dep.dll", "https://another.mirror/example-dep.dll"],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-also-totally-a-real-hash")
		}],
		discord: Some("a-discord-invite-url"),
		poster_image: Some("a-banner-image-url"),
		rss: Some("an-rss-feed-url"),
		webpage: Some("a-webpage-url"),
		forums: Some("a-forum-url")
	});
}

fn parse_profiles<'a>(profiles: &roxmltree::Node::<'a, '_>, result: &mut Manifest<'a>) {
	for node in profiles.children() {
		if node.is_element() {
			println!("profile {:?}", node);
			match node.tag_name().name() {
				TQ_TAG_LAUNCH => result.profiles.push(ManifestProfile::<'a> {
					exec: node.attribute(TQ_ATTR_EXEC).unwrap(),
					name: node.text().unwrap(),
					params: node.attribute(TQ_ATTR_PARAMS),
					order: node.attribute(TQ_ATTR_ORDER).and_then(|a: &str| a.parse::<u8>().ok()),
					architecture: node.attribute(TQ_ATTR_ARCH)
				}),
				_ => ()
			}
		}
	}
}