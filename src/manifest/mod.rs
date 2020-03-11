// --- Dependencies
extern crate toml;

// --- Modules
mod manifest_spec;

// --- Imports
use manifest_spec::Manifest;

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
			path: "example-app.exe".to_owned(),
			url: vec!["https://some.example.app/example-app.exe".to_owned()],
			size: Some(0),
			md5: None,
			sha1: None,
			sha256: Some("this-is-totally-a-real-hash".to_owned())
		},
		manifest_spec::ManifestFile {
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
	};
	println!("{}", toml::to_string::<manifest_spec::vg_1_0::Manifest_VG_1_0>(&a.into()).unwrap())
}

pub fn parse_manifest(manifest: &str) {
	manifest_spec::tq::parse_manifest(manifest);
}