// --- Modules
pub mod config;
pub mod manifest;

fn main() {
    let cfg = config::get_config();

    println!("{:#?}", cfg);

    config::write_config_file(&cfg).unwrap();
	// // Get Manifest
	// let http_manifest = ureq::get(&std::env::var("VG_MANIFEST").unwrap()).call();
	// if http_manifest.ok() {
    //     println!("Got it!");
    //     let manifest = manifest::deserialize_manifest(&http_manifest.into_string().unwrap()).unwrap();
    //     println!("{:#?}", manifest);
    //     let serialized_as_toml = manifest::manifest_spec::vg_1_0::serialize_manifest(&manifest).unwrap();
    //     println!("{}", serialized_as_toml);
    //     let parsed_as_toml = manifest::deserialize_manifest(&serialized_as_toml).unwrap();
    //     println!("{:#?}", parsed_as_toml);
    // }
    // else {
    //     panic!("Manifest get failed.");
    // }
}
