// --- Dependencies
extern crate ureq;

// --- Modules
mod manifest;

fn main() {
    // Get Manifest
    let http_manifest = ureq::get(&std::env::var("VG_MANIFEST").unwrap())
        .call();
    if http_manifest.ok() {
        println!("Got it!");
        manifest::parse_manifest(&http_manifest.into_string().unwrap())
    } else {
        panic!("Manifest get failed.");
    }
}