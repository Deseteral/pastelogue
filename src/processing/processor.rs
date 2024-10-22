use crate::exif::exif_date_time::ExifDateTime;
use crate::exif::extract_metadata::PhotoMetadata;
use crate::processing::check_file::{check_file, CheckStatus};
use crate::processing::fs_operations::{create_dirs, move_file};
use crate::processing::scan_dir::scan_dir;
use std::path::{Path, PathBuf};

pub struct LibraryProcessor {
    root_path: PathBuf,
    files: Vec<PathBuf>,
    current: usize,
    next: usize,
}

pub struct ProcessingInfo {
    pub current: u32,
    pub total: u32,
    pub original_path: PathBuf,
    pub path: PathBuf,
    pub status: ProcessingStatus,
    pub exif_data: Option<SimpleExifData>,
}

pub struct SimpleExifData {
    pub created_at: ExifDateTime,
}

#[derive(PartialEq)]
pub enum ProcessingStatus {
    Ok,
    BadMetadata,
}

impl LibraryProcessor {
    pub fn new(root_path: &Path) -> LibraryProcessor {
        let files = scan_dir(&root_path);
        LibraryProcessor {
            root_path: root_path.to_owned(),
            files,
            current: 0,
            next: 1,
        }
    }

    fn process_current_file(&self) -> ProcessingInfo {
        let current_path = &self.files[self.current];

        let mut info = ProcessingInfo {
            current: (self.current + 1) as u32,
            total: self.len() as u32,
            original_path: current_path.to_path_buf(),
            path: current_path.to_path_buf(),
            status: ProcessingStatus::Ok,
            exif_data: None,
        };

        let metadata = PhotoMetadata::from_file(&current_path);
        if metadata.is_err() {
            info.status = ProcessingStatus::BadMetadata;
            return info;
        };
        let metadata = metadata.unwrap();

        let status = check_file(&current_path, &metadata, &self.root_path);
        if let CheckStatus::Wrong(correct_path) = status {
            create_dirs(&correct_path);
            move_file(&current_path.to_path_buf(), &correct_path);
            info.path = correct_path;
        }

        info.exif_data = Some(SimpleExifData {
            created_at: metadata.datetime,
        });

        info
    }
}

impl Iterator for LibraryProcessor {
    type Item = ProcessingInfo;

    fn next(&mut self) -> Option<ProcessingInfo> {
        let info = if self.current >= self.files.len() {
            None
        } else {
            let info = self.process_current_file();
            Some(info)
        };

        self.current = self.next;
        self.next += 1;

        info
    }
}

impl ExactSizeIterator for LibraryProcessor {
    fn len(&self) -> usize {
        self.files.len()
    }
}
