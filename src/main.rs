// --- Modules
pub mod config;
pub mod manifest;

fn main() {
    let def = config::get_default_config();
    let cfg = config::read_config_file();

    println!("{:#?}", def);
    println!("{:#?}", cfg);

    config::write_config_file(&def).unwrap();
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
