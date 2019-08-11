use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};

pub struct PreflightElement {
    current_path: PathBuf,
    correct_path: PathBuf,
}

fn file_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn file_is_photo(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.ends_with(".jpg"))
        .unwrap_or(false)
}

pub fn run_preflight(root_path: &str) -> Vec<PreflightElement> {
    let mut list: Vec<PreflightElement> = Vec::new();

    let acceptable_files = WalkDir::new(root_path)
        .into_iter()
        .filter_entry(|e| !file_is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| file_is_photo(e));

    for entry in acceptable_files {
        println!("{}", entry.path().display());
    }

    list
}
