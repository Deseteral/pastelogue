use std::env;
use std::path::PathBuf;

mod scan_dir;
mod processing;
mod check_file;
mod fs_operations;
mod extract_metadata;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_path = PathBuf::from(&args[1]);

    processing::process_dir(&root_path);
}
