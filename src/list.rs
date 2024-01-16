use crate::common::{projection_dir, template_dirs};
use std::fs;

pub fn list_templates() {
    let projection_dir = projection_dir();
    if !projection_dir.exists() {
        fs::create_dir(&projection_dir).expect("Couldn't create projection dir");
    }
    // Get all directories
    let dirs = template_dirs(projection_dir);
    for dir in dirs {
        println!("{}", dir.file_name().unwrap().to_str().unwrap());
    }
}
