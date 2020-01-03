use std::env;
use std::path::PathBuf;

use pastelogue::CatalogueProcessor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_path = PathBuf::from(&args[1]);

    // process_dir(&root_path);
}
