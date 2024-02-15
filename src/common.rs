use simple_home_dir::home_dir;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn template_dirs(dir: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("Unable to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

pub fn projection_dir() -> PathBuf {
    home_dir().expect("Cannot find home dir").join("projection")
}
pub fn cwd() -> PathBuf {
    std::env::current_dir().expect("Unable to get current working directory")
}
pub fn is_dir_empty(dir: &Path) -> bool {
    let files = dir.read_dir().expect("Unable to access directory");
    files.count() == 0
}
