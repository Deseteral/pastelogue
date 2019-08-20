use std::path::Path;
use crate::extract_metadata::PhotoMetadata;
use crate::check_file::{check_file, CheckStatus};
use crate::scan_dir::scan_dir;
use crate::fs_operations::{create_dirs, move_file};

pub fn process_dir(root_path: &Path) {
    let files = scan_dir(&root_path);
    println!("Scanning completed with {} files to process", files.len());

    for file_path in files {
        process_file(&file_path, &root_path);
    }
}

fn process_file(file_path: &Path, root_path: &Path) {
    let metadata = PhotoMetadata::from_file(&file_path);
    let status = check_file(&file_path, metadata, &root_path);

    if let CheckStatus::Wrong(correct_path) = status {
        create_dirs(&correct_path);
        move_file(&file_path.to_path_buf(), &correct_path);

        println!("Correctly processed file: {}", &file_path.display());
    }
}
