use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use crate::check_file;
use crate::extract_metadata;

#[derive(Debug)]
pub struct PreflightElement {
    pub current_path: PathBuf,
    pub correct_path: PathBuf,
}

fn file_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn file_is_photo(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.to_lowercase().ends_with(".jpg") || s.to_lowercase().ends_with(".jpeg")) // TODO: Add more file types
        .unwrap_or(false)
}

pub fn run_preflight(root_path: &Path) -> Vec<PreflightElement> {
    let mut list: Vec<PreflightElement> = Vec::new();

    let acceptable_files = WalkDir::new(root_path)
        .into_iter()
        .filter_entry(|e| !file_is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| file_is_photo(e));

    for entry in acceptable_files {
        let entry_path: &Path = entry.path();

        let metadata = extract_metadata::PhotoMetadata::from_file(&entry_path);
        let status = check_file::check_file(&entry_path, metadata, &root_path);

        if let check_file::CheckStatus::Wrong(correct_path) = status {
            list.push(PreflightElement {
                current_path: entry_path.to_path_buf(),
                correct_path,
            });
        }
    }

    list
}
