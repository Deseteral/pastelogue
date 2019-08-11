use std::env;
use std::path::PathBuf;

mod extract_metadata;
mod check_file;
mod preflight;
mod processing;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = PathBuf::from(&args[1]);

    let list = preflight::run_preflight(&path);

    println!("Preflight completed with {} files to process", list.len());

    if !list.is_empty() {
        processing::run_processing(list);
    }
}
