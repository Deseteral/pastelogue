use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::preflight::PreflightElement;

fn create_dirs(path: &PathBuf) {
    let directory_path = path.parent().unwrap();
    match create_dir_all(&directory_path) {
        Ok(_) => println!("Created directory: ${}", &directory_path.display()),
        Err(_) => panic!("Failed to create directory path: {}", &directory_path.display()),
    };
}

pub fn run_processing(list: Vec<PreflightElement>) {
    for element in list {
        create_dirs(&element.correct_path);


        // dbg!(&element.correct_path.parent());
    }
}
