use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let exiv2_exec = Path::new(&cargo_manifest_dir).join("build").join("exiv2").join("exiv2json");
    let out_dir = Path::new(&cargo_manifest_dir).join("target").join(&profile);

    fs::copy(exiv2_exec, out_dir.join("exiv2json")).unwrap();
}
