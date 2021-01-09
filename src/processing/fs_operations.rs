use std::fs;
use std::path::Path;

pub fn create_dirs(path: &Path) {
    let directory_path = path.parent().unwrap();
    match fs::create_dir_all(&directory_path) {
        Ok(_) => {}
        Err(_) => panic!(
            "Failed to create directory path: {}",
            &directory_path.display()
        ),
    };
}

pub fn move_file(old_path: &Path, new_path: &Path) {
    // TODO: Why can't I just move the file? Why needless copy?
    match fs::copy(old_path, new_path) {
        Ok(_) => {}
        Err(err) => panic!(
            "Could not copy file from '{}' to '{}'; {}",
            &old_path.display(),
            &new_path.display(),
            &err,
        ),
    };

    match fs::remove_file(old_path) {
        Ok(_) => {}
        Err(_) => panic!("Could not remove file: {}", &old_path.display()),
    };
}

// TODO: Add dry run mode with human readable output (terminal output along with HTML export?)
