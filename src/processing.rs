use crate::check_file::{check_file, CheckStatus};
use crate::extract_metadata::PhotoMetadata;
use crate::fs_operations::{create_dirs, move_file};
use crate::scan_dir::scan_dir;
use std::path::{Path, PathBuf};

pub struct CatalogueProcessor {
    root_path: PathBuf,
    files: Vec<PathBuf>,
    current: usize,
    next: usize,
}

#[derive(Debug)]
pub struct ProcessingInfo {
    pub current: u32,
    pub total: u32,
    pub path: PathBuf,
}

impl CatalogueProcessor {
    pub fn new(root_path: &Path) -> CatalogueProcessor {
        let files = scan_dir(&root_path);
        CatalogueProcessor {
            root_path: root_path.to_owned(),
            files,
            current: 0,
            next: 1,
        }
    }

    fn process_current_file(&self) -> ProcessingInfo {
        let current_path = &self.files[self.current];

        let info = ProcessingInfo {
            current: self.current as u32,
            total: self.len() as u32,
            path: current_path.to_path_buf(),
        };

        let metadata = PhotoMetadata::from_file(&current_path);
        if metadata.is_err() {
            return info;
        };
        let metadata = metadata.unwrap();

        let status = check_file(&current_path, &metadata, &self.root_path);

        if let CheckStatus::Wrong(correct_path) = status {
            create_dirs(&correct_path);
            move_file(&current_path.to_path_buf(), &correct_path);
        }

        info
    }
}

impl Iterator for CatalogueProcessor {
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

impl ExactSizeIterator for CatalogueProcessor {
    fn len(&self) -> usize {
        self.files.len()
    }
}
