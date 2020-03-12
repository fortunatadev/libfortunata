// --- Dependencies
extern crate toml;

// --- Modules
mod manifest_spec;

// --- Imports
use manifest_spec::Manifest;

// pub fn manifest_as_toml() {
// 	let a = manifest_spec::vg_1_0::Manifest_VG_1_0 {
// 		version: "vg-1.0",
// 		label: "Example Manifest",
// 		profiles: vec![manifest_spec::vg_1_0::MF_Profile_VG_1_0 {
// 			name: "Example App",
// 			exec: "example-app.exe",
// 			params: Some("--be-super-awesome"),
// 			order: Some(0),
// 			architecture: Some("x64")
// 		}],
// 		files: vec![manifest_spec::vg_1_0::MF_File_VG_1_0 {
// 			path: "example-app.exe",
// 			url: vec!["https://some.example.app/example-app.exe"],
// 			size: Some(0),
// 			md5: None,
// 			sha1: None,
// 			sha256: Some("this-is-totally-a-real-hash")
// 		},
// 		manifest_spec::vg_1_0::MF_File_VG_1_0 {
// 			path: "example-dep.dll",
// 			url: vec!["https://some.example.app/example-dep.dll", "https://another.mirror/example-dep.dll"],
// 			size: Some(0),
// 			md5: None,
// 			sha1: None,
// 			sha256: Some("this-is-also-totally-a-real-hash")
// 		}],
// 		discord: Some("a-discord-invite-url"),
// 		poster_image: Some("a-banner-image-url"),
// 		rss: Some("an-rss-feed-url"),
// 		webpage: Some("a-webpage-url"),
// 		forums: Some("a-forum-url")
// 	};
// 	println!("{}", toml::to_string::<manifest_spec::vg_1_0::Manifest_VG_1_0>(&a.into()).unwrap())
// }

pub fn parse_manifest(manifest: &str) {
	let result = manifest_spec::tq::parse_manifest(&manifest).unwrap();
	println!("{:#?}", result);
	manifest_to_toml(&result);
}

pub fn manifest_to_toml<'a>(manifest: &'a Manifest) {
	let versioned_manifest: manifest_spec::vg_1_0::Manifest_VG_1_0 = manifest.into();
	let result_as_toml = toml::to_string(&versioned_manifest).unwrap();
	println!("{}", result_as_toml);
}