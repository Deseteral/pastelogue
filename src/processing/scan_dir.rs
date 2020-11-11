use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

// TODO: Add more file types.
static SUPPORTED_FILE_EXTENSIONS: &'static [&str] = &[".jpg", ".jpeg"];

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
        .map(|filename| file_has_supported_file_extension(filename))
        .unwrap_or(false)
}

fn file_has_supported_file_extension(filename: &str) -> bool {
    SUPPORTED_FILE_EXTENSIONS
        .iter()
        .any(|ext| filename.to_lowercase().ends_with(*ext))
}

// TODO: Add support for video. For scraping metadata ffprobe from ffmpeg family should be enough for the job.
//       Also it looks like exiv2 has some support for video - maybe it will be enough (https://dev.exiv2.org/projects/exiv2/wiki/Supported_video_formats)
