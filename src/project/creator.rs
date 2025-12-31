use std::fs;
pub fn prepare_dirs() -> Option<std::io::Error> {
    let dirs = vec!["./build", "./build/cache", "./build/fixed"];

    for dir in dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            return Some(e);
        }
    }

    None
}

pub fn clean() -> Option<std::io::Error> {
    if let Err(e) = fs::remove_dir_all("build/") {
        return Some(e);
    }

    None
}
