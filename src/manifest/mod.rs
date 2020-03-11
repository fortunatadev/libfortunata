// --- Dependencies
extern crate quick_xml;
extern crate toml;

// --- Modules
mod manifest_spec;

// --- Imports
use manifest_spec::Manifest;
use quick_xml::{de, se};

pub fn tqmanifest_as_xml() {
	let a = manifest_spec::tq::Manifest_TQ {
		label: "Example Manifest".to_owned(),
		profiles: vec![manifest_spec::tq::MF_Profile_TQ {
			name: "Example App".to_owned(),
			exec: "example-app.exe".to_owned(),
			params: Some("--be-super-awesome".to_owned()),
			order: Some("0".to_owned()),
			architecture: Some("x64".to_owned())
		}],
		filelist: vec![manifest_spec::tq::MF_File_TQ {
			name: "example-app.exe".to_owned(),
			url: vec!["https://some.example.app/example-app.exe".to_owned()],
			size: Some("0".to_owned()),
			md5: Some("this-is-totally-a-real-hash".to_owned()),
		},
		manifest_spec::tq::MF_File_TQ {
			name: "example-dep.dll".to_owned(),
			url: vec!["https://some.example.app/example-dep.dll".to_owned(), "https://another.mirror/example-dep.dll".to_owned()],
			size: Some("0".to_owned()),
			md5: Some("this-is-also-totally-a-real-hash".to_owned()),
		}],
		launchers: vec![manifest_spec::tq::MF_Launcher_TQ {
			id: "tequila".to_owned(),
			url: vec!["tequila.com".to_owned()],
			size: "1".to_owned(),
			md5: "sup".to_owned(),
			version: "1.0".to_owned()
		}],
		webpage: Some("a-webpage-url".to_owned()),
		forums: Some("a-forum-url".to_owned()),
		poster_image: Some("a-banner-image-url".to_owned())
	};
	println!("{}", se::to_string(&a).unwrap())
}

pub fn manifest_as_xml() {
	let a = manifest_spec::vg_1_0::Manifest_VG_1_0 {
		version: "vg-1.0".to_owned(),
		label: "Example Manifest".to_owned(),
		profiles: vec![manifest_spec::vg_1_0::MF_Profile_VG_1_0 {
			name: "Example App".to_owned(),
			exec: "example-app.exe".to_owned(),
			params: None,
			order: None,
			architecture: None
		}],
		files: vec![manifest_spec::vg_1_0::MF_File_VG_1_0 {
			name: "example-app.exe".to_owned(),
			url: vec!["https://some.example.app/example-app.exe".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-totally-a-real-hash".to_owned())
		},
		manifest_spec::vg_1_0::MF_File_VG_1_0 {
			name: "example-dep.dll".to_owned(),
			url: vec!["https://some.example.app/example-dep.dll".to_owned(), "https://another.mirror/example-dep.dll".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-also-totally-a-real-hash".to_owned())
		}],
		discord: None,
		poster_image: None,
		rss: None,
		webpage: None,
		forums: None
	};
	println!("{}", se::to_string(&a).unwrap())
}

pub fn manifest_as_toml() {
	let a = Manifest {
		version: "vg-1.0".to_owned(),
		label: "Example Manifest".to_owned(),
		profiles: vec![manifest_spec::ManifestProfile {
			name: "Example App".to_owned(),
			exec: "example-app.exe".to_owned(),
			params: Some("--be-super-awesome".to_owned()),
			order: Some(0),
			architecture: Some("x64".to_owned())
		}],
		files: vec![manifest_spec::ManifestFile {
			name: "example-app.exe".to_owned(),
			url: vec!["https://some.example.app/example-app.exe".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-totally-a-real-hash".to_owned())
		},
		manifest_spec::ManifestFile {
			name: "example-dep.dll".to_owned(),
			url: vec!["https://some.example.app/example-dep.dll".to_owned(), "https://another.mirror/example-dep.dll".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-also-totally-a-real-hash".to_owned())
		}],
		launchers: None,
		discord: Some("a-discord-invite-url".to_owned()),
		poster_image: Some("a-banner-image-url".to_owned()),
		rss: Some("an-rss-feed-url".to_owned()),
		webpage: Some("a-webpage-url".to_owned()),
		forums: Some("a-forum-url".to_owned())
	};
	println!("{}", toml::to_string::<manifest_spec::vg_1_0::Manifest_VG_1_0>(&a.into()).unwrap())
}

pub fn parse_manifest(manifest: &str) {
	println!();
	// Whitespace breaks quick-xml/serde xml deserialization
	let cleanManifest: String = manifest.replace("<?xml version=\"1.0\" ?>", "").chars().filter(|&c| c != '\t' && c != '\n' && c != '\r').collect();
	//println!("{}", cleanManifest);
	let parsed: manifest_spec::tq::Manifest_TQ = de::from_str(&cleanManifest).unwrap();
}