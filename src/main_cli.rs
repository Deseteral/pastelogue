use std::env;
use std::path::PathBuf;

use pastelogue::{process_library, ProcessingConfig};

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_path = PathBuf::from(&args[1]);

    let config = ProcessingConfig { dry_run: true };
    process_library(&root_path, config);
}
