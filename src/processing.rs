use std::fs;
use std::path::PathBuf;
use crate::preflight::PreflightElement;

fn create_dirs(path: &PathBuf) {
    let directory_path = path.parent().unwrap();
    match fs::create_dir_all(&directory_path) {
        Ok(_) => println!("Created directory: ${}", &directory_path.display()),
        Err(_) => panic!("Failed to create directory path: {}", &directory_path.display()),
    };
}

fn move_file(old_path: &PathBuf, new_path: &PathBuf) {
    match fs::copy(old_path, new_path) {
        Ok(_) => {},
        Err(_) => panic!("Could not copy file: {}", &old_path.display()),
    };

    match fs::remove_file(old_path) {
        Ok(_) => {},
        Err(_) => panic!("Could not remove file: {}", &old_path.display()),
    }
}

pub fn run_processing(list: Vec<PreflightElement>) {
    for element in list {
        create_dirs(&element.correct_path);
        move_file(&element.current_path, &element.correct_path);
    }
}
