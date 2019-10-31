use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let env_profile = env::var("PROFILE").unwrap();
    let env_cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_dir = Path::new(&env_cargo_manifest_dir)
        .join("target")
        .join(&env_profile);
    let exiv2_exec = Path::new(&env_cargo_manifest_dir)
        .join("build")
        .join("exiv2")
        .join("exiv2json");

    fs::copy(exiv2_exec, out_dir.join("exiv2json")).unwrap();
}
