/// --- Dependencies
extern crate roxmltree;

// --- Imports
use super::{Manifest, ManifestError};
use roxmltree::Document;

// --- Consts
const TQ_VERSION: &str		= "tq";
const TQ_TAG_MANIFEST: &str = "manifest";
const TQ_TAG_PROFILES: &str = "profiles";
const TQ_TAG_LAUNCH: &str	= "launch";
const TQ_TAG_FILELIST: &str = "filelist";
const TQ_TAG_FILE: &str		= "file";

pub fn parse_manifest(manifest: &str) -> Result<Manifest, ManifestError> {
	// A Tequila manifest should only have one root element named `manifest`.
	let xml_doc = Document::parse(manifest)?;
	let xml_manifest = xml_doc.root().children().next().unwrap();
	if !xml_manifest.is_element() || xml_manifest.tag_name().name() != TQ_TAG_MANIFEST {
		return Err(ManifestError::Invalid);
	}

	// Build the start of our return object.
	let mut result: Manifest = {
		verison: TQ_VERSION.to_owned(),
		label: "None".to_owned(),
	};

	// Iterate over root children. Non-matching elements are silently discarded.
	for node in xml_manifest.children() {
		if node.is_element() {
			println!("{:?}", node);
			match node.tag_name().name() {
				TQ_TAG_PROFILES => parse_profiles(node),
				_ => ()
			}
		}
	}

	return Ok(super::Manifest {
		version: "vg-1.0".to_owned(),
		label: "Example Manifest".to_owned(),
		profiles: vec![super::ManifestProfile {
			name: "Example App".to_owned(),
			exec: "example-app.exe".to_owned(),
			params: Some("--be-super-awesome".to_owned()),
			order: Some(0),
			architecture: Some("x64".to_owned())
		}],
		files: vec![super::ManifestFile {
			path: "example-app.exe".to_owned(),
			url: vec!["https://some.example.app/example-app.exe".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-totally-a-real-hash".to_owned())
		},
		super::ManifestFile {
			path: "example-dep.dll".to_owned(),
			url: vec!["https://some.example.app/example-dep.dll".to_owned(), "https://another.mirror/example-dep.dll".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-also-totally-a-real-hash".to_owned())
		}],
		discord: Some("a-discord-invite-url".to_owned()),
		poster_image: Some("a-banner-image-url".to_owned()),
		rss: Some("an-rss-feed-url".to_owned()),
		webpage: Some("a-webpage-url".to_owned()),
		forums: Some("a-forum-url".to_owned())
	});
}

fn parse_profiles(profiles: roxmltree::Node) {
	for node in profiles.children() {
		if node.is_element() {
			println!("profile {:?}", node);
			match node.tag_name().name() {
				TQ_TAG_LAUNCH => (),
				_ => ()
			}
		}
	}
}