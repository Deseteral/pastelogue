use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn scan_dir(dir_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_entry(|e| !file_is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| file_is_photo(e))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn file_is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn file_is_photo(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.to_lowercase().ends_with(".jpg") || s.to_lowercase().ends_with(".jpeg")) // TODO: Add more file types.
        .unwrap_or(false)
}

// TODO: Add support for video. For scraping metadata ffprobe from ffmpeg family should be enough for the job.
