use std::fs;
use std::path::PathBuf;

pub fn create_dirs(path: &PathBuf) {
    let directory_path = path.parent().unwrap();
    match fs::create_dir_all(&directory_path) {
        Ok(_) => {},
        Err(_) => panic!("Failed to create directory path: {}", &directory_path.display()),
    };
}

pub fn move_file(old_path: &PathBuf, new_path: &PathBuf) {
    match fs::copy(old_path, new_path) {
        Ok(_) => {},
        Err(_) => panic!("Could not copy file: {}", &old_path.display()),
    };

    match fs::remove_file(old_path) {
        Ok(_) => {},
        Err(_) => panic!("Could not remove file: {}", &old_path.display()),
    }
}
