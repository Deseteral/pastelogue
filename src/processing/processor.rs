use crate::exif::extract_metadata::PhotoMetadata;
use crate::processing::check_file::{check_file, CheckStatus};
use crate::processing::scan_dir::scan_dir;
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

use super::fs_operations::{create_dirs, move_file};

#[derive(Debug)]
enum TransformOperation {
    NoEffect,
    Change(PathBuf),
    MetadataReadError,
}

#[derive(Debug)]
struct FileDestiny {
    file_path: PathBuf,
    transform_operation: TransformOperation,
}

impl FileDestiny {
    fn predicted_file_path(&self) -> PathBuf {
        // TODO: Do not clone, return borrows
        match &self.transform_operation {
            TransformOperation::Change(next_path) => next_path.clone(),
            _ => self.file_path.clone(),
        }
    }
}

pub fn process_library(library_path: &Path) {
    // Prepare list of all media files in library
    let files = scan_dir(&library_path);

    // Generate first iteration of transform list from media file list
    // TODO: Could be multithreaded for performance boost
    let mut file_ops = build_file_ops_from_metadata(files, library_path);

    // Hande media files with exactly the same date that would otherwise be overwritten
    resolve_duplicate_files(&mut file_ops);

    // Move files on disk
    for file_destiny in &file_ops {
        if let TransformOperation::Change(correct_path) = &file_destiny.transform_operation {
            create_dirs(correct_path);
            move_file(&file_destiny.file_path, correct_path);
        }
    }

    dbg!(&file_ops);
}

fn build_file_ops_from_metadata(files: Vec<PathBuf>, library_path: &Path) -> Vec<FileDestiny> {
    let mut file_ops: Vec<FileDestiny> = Vec::new();

    for file_path in files {
        let transform_operation = match PhotoMetadata::from_file(&file_path) {
            Ok(metadata) => {
                let status = check_file(&file_path, &metadata, &library_path); // TODO: check_file and status are meh names
                let transform_operation = match status {
                    CheckStatus::Wrong(correct_path) => TransformOperation::Change(correct_path),
                    CheckStatus::Correct => TransformOperation::NoEffect,
                };

                transform_operation
            }
            Err(_) => TransformOperation::MetadataReadError,
        };

        file_ops.push(FileDestiny {
            file_path,
            transform_operation,
        });
    }

    file_ops
}

fn get_repeated_paths(file_ops: &Vec<FileDestiny>) -> Vec<PathBuf> {
    let mut predicted_paths_count: HashMap<PathBuf, u32> = HashMap::new();
    let predicted_paths: Vec<PathBuf> =
        file_ops.iter().map(|fd| fd.predicted_file_path()).collect();
    for predicted_file_path in predicted_paths {
        let count = match predicted_paths_count.get(&predicted_file_path) {
            Some(&current_count) => current_count + 1,
            _ => 1,
        };

        predicted_paths_count.insert(predicted_file_path, count);
    }

    let mut repeated_paths: Vec<PathBuf> = Vec::new();
    for (file_path, occurence_count) in predicted_paths_count {
        if occurence_count > 1 {
            repeated_paths.push(file_path);
        }
    }

    repeated_paths
}

fn resolve_duplicate_files(file_ops: &mut Vec<FileDestiny>) {
    // TODO: Check file checksums and handle duplicate files
    //       Media files with exactly the same date that are different (different checksums) should have _1, _2 etc. suffix
    //       Media files with exactly the same date and checksum should be deduplicated (only one copy of file should remain)
    for repeated_file in &get_repeated_paths(&file_ops) {
        let mut occurence_counter: u32 = 1;

        for file_destiny in &mut *file_ops {
            let predicted_file_path = file_destiny.predicted_file_path();
            if predicted_file_path == *repeated_file {
                let mut new_path = predicted_file_path.clone();
                add_counter_to_filename(&mut new_path, &occurence_counter);

                file_destiny.transform_operation = TransformOperation::Change(new_path);
                occurence_counter += 1;
            }
        }
    }
}

fn add_counter_to_filename(file_path: &mut PathBuf, counter: &u32) {
    let counter_oss: OsString = counter.to_string().into();
    let mut new_filename: OsString = OsString::new();

    new_filename.push(file_path.file_stem().unwrap());
    new_filename.push("_");
    new_filename.push(counter_oss);
    new_filename.push(".");
    new_filename.push(file_path.extension().unwrap());

    file_path.set_file_name(new_filename);
}
