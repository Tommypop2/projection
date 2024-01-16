use clap::Error;

use crate::common::{projection_dir, template_dirs};
use std::{fs, path::Path};
fn print_dir_name(start: &str, dir: &Path) {
    println!("{} {}", start, dir.file_name().unwrap().to_str().unwrap());
}
pub fn list_templates() -> Result<(), Error> {
    let projection_dir = projection_dir();
    if !projection_dir.exists() {
        fs::create_dir(&projection_dir).expect("Couldn't create projection dir");
    }
    // Get all directories
    let dirs = template_dirs(projection_dir);
    for i in 0..dirs.len() {
        print_dir_name(
            if i == dirs.len() - 1 {
                "└──"
            } else {
                "├──"
            },
            &dirs[i],
        )
    }
    // Err(clap::Error::new(error::ErrorKind::DisplayHelp))
    Ok(())
}
