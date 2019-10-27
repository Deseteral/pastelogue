use std::env;
use std::path::PathBuf;

mod date_time;
mod check_file;
mod extract_metadata;
mod fs_operations;
mod processing;
mod scan_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_path = PathBuf::from(&args[1]);

    processing::process_dir(&root_path);
}
