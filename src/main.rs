use std::env;
use std::path::PathBuf;

mod extract_metadata;
mod check_file;
mod preflight;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = PathBuf::from(&args[1]);

    let list = preflight::run_preflight(&path);
    dbg!(&list);
}
