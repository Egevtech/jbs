use std::{fs, path::Path};
pub fn prepare_dirs() -> Option<std::io::Error> {

    let dirs = vec!["./build", "./build/cache", "./build/fixed"];

    for dir in dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            return Some(e)
        }
    }

    None
}