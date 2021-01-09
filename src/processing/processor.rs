use super::fs_operations::{create_dirs, move_file};
use crate::exif::extract_metadata::PhotoMetadata;
use crate::processing::check_file::{check_file_path_with_metadata, FilePathCheckStatus};
use crate::processing::scan_dir::scan_dir;
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Debug)]
pub struct ProcessingConfig {
    pub dry_run: bool,
}

pub fn process_library(library_path: &Path, config: ProcessingConfig) -> ProcessingResult {
    let debug_scan_time = Instant::now();

    // Prepare list of all media files in library
    let files = scan_dir(&library_path);

    println!("scan: {} ms", debug_scan_time.elapsed().as_millis());

    let debug_processing_time = Instant::now();

    // Generate first iteration of transform list from media file list
    // TODO: Could be multithreaded for performance boost
    let mut file_ops: Vec<FileOperation> = Vec::new();
    for file_path in &files {
        let file_operation = FileOperation::build_from_metadata(file_path, library_path);
        file_ops.push(file_operation);
    }

    println!(
        "processing: {} ms",
        debug_processing_time.elapsed().as_millis()
    );

    let debug_deduplicate_time = Instant::now();

    // Hande media files with exactly the same date that would otherwise be overwritten
    handle_duplicate_files(&mut file_ops);

    println!(
        "duplicate handling: {} ms",
        debug_deduplicate_time.elapsed().as_millis()
    );

    let debug_fs_time = Instant::now();

    // Move files on disk
    if !config.dry_run {
        for file_operation in &file_ops {
            file_operation.execute_operation();
        }
    }

    println!("fs ops: {} ms", debug_fs_time.elapsed().as_millis());

    ProcessingResult { file_ops }
}

pub struct ProcessingResult {
    pub file_ops: Vec<FileOperation>,
}

#[derive(Debug)]
pub struct FileOperation {
    pub original_file_path: PathBuf,
    pub transform_operation: TransformOperation,
}

#[derive(Debug)]
pub enum TransformOperation {
    NoEffect,
    Change(PathBuf),
    MetadataReadError,
}

impl FileOperation {
    fn build_from_metadata(file_path: &Path, library_path: &Path) -> FileOperation {
        let transform_operation = match PhotoMetadata::from_file(&file_path) {
            Ok(metadata) => {
                let status = check_file_path_with_metadata(&file_path, &metadata, &library_path);
                let transform_operation = match status {
                    FilePathCheckStatus::Wrong(correct_path) => {
                        TransformOperation::Change(correct_path)
                    }
                    FilePathCheckStatus::Correct => TransformOperation::NoEffect,
                };

                transform_operation
            }
            Err(_) => TransformOperation::MetadataReadError,
        };

        FileOperation {
            original_file_path: file_path.to_owned(),
            transform_operation,
        }
    }

    fn predicted_file_path(&self) -> PathBuf {
        match &self.transform_operation {
            TransformOperation::Change(next_path) => next_path.clone(),
            _ => self.original_file_path.clone(),
        }
    }

    fn execute_operation(&self) {
        if let TransformOperation::Change(correct_path) = &self.transform_operation {
            create_dirs(correct_path);
            move_file(&self.original_file_path, correct_path);
        }
    }
}

fn get_repeated_paths(file_ops: &Vec<FileOperation>) -> Vec<PathBuf> {
    let mut predicted_paths_count: HashMap<PathBuf, u32> = HashMap::new();
    let predicted_paths = file_ops.iter().map(|fd| fd.predicted_file_path());

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

fn handle_duplicate_files(file_ops: &mut Vec<FileOperation>) {
    // TODO: Check file checksums and handle duplicate files
    //       Media files with exactly the same date that are different (different checksums) should have _1, _2 etc. suffix
    //       Media files with exactly the same date and checksum should be deduplicated (only one copy of file should remain)
    for repeated_file in &get_repeated_paths(&file_ops) {
        let mut occurence_counter: u32 = 1;

        for file_operation in &mut *file_ops {
            let predicted_file_path = file_operation.predicted_file_path();
            let is_repeated_file = predicted_file_path == *repeated_file;

            if is_repeated_file {
                let mut new_path = predicted_file_path.clone();
                add_counter_to_filename(&mut new_path, &occurence_counter);
                file_operation.transform_operation = TransformOperation::Change(new_path);

                occurence_counter += 1;
            }
        }
    }
}

fn add_counter_to_filename(file_path: &mut PathBuf, counter: &u32) {
    let counter_oss: OsString = counter.to_string().into();

    // TODO: Check if there is a way to format OsString and simplify statements below
    let mut new_filename = OsString::new();
    new_filename.push(file_path.file_stem().unwrap());
    new_filename.push("_");
    new_filename.push(counter_oss);
    new_filename.push(".");
    new_filename.push(file_path.extension().unwrap());

    file_path.set_file_name(new_filename);
}
