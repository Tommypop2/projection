use std::{fs, path::PathBuf};

use simple_home_dir::home_dir;

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
