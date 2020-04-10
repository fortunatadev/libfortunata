// --- Modules
mod config;
mod file_sync;
mod manifest;

fn main() {
    println!("Initializing Vanguard...");

    // Get config
    let cfg = config::read_config_file(None).unwrap();
    warn_unsafe_config(&cfg);
    
	// Get Manifest
	let http_manifest = ureq::get(&cfg.manifests[0].url).call();
	if http_manifest.ok() {
        println!("Got manifest.");
        let manifest = manifest::deserialize_manifest(&http_manifest.into_string().unwrap()).unwrap();
        println!("Starting file sync...");
        file_sync::sync_streaming(&manifest, &cfg.manifests[0]).unwrap();
    }
    else {
        panic!("Manifest get failed.");
    }
}

/// Screams angrily at users who want to hurt themselves and/or their systems.
fn warn_unsafe_config(config: &config::Config) {
    for mf_cfg in &config.manifests {
        if mf_cfg.allow_insecure_patching {
            println!("WARNING: allow_insecure_patching enabled for manifest: {}", mf_cfg.url);
            println!("         Insecure HTTP download mirrors may be used, which could possibly be used to send malicious code.");
        }
        if mf_cfg.ignore_hash_check {
            println!("WARNING: ignore_hash_check enabled for manifest: {}", mf_cfg.url);
            println!("         File contents will not be validated, possibly allowing broken or malicious files to be saved.");
        }
        if mf_cfg.allow_unsafe_file_paths {
            println!("WARNING: allow_unsafe_file_paths enabled for manifest: {}", mf_cfg.url);
            println!("         This is __extremely unsafe__ and allows Vanguard to potentially modify any file on your system.");
        }
    }
}
