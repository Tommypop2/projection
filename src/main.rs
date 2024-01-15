use std::{fs, path::PathBuf};

use clap::Parser;
use fs_extra::dir::CopyOptions;
static PROJECTION_DIR: &str = "C:/Users/thoma/projection";
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Easily scaffold projects from templates
struct Args {
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    template_name: String,
}

fn main() {
    let args = Args::parse();
    // Get all directories
    let dirs: Vec<PathBuf> = fs::read_dir(PROJECTION_DIR)
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
        .collect();
    // Find the template
    let template_dir = dirs
        .iter()
        .find(|dir| dir.ends_with(&args.template_name))
        .expect("No template found");
    // Copy template to current working directory

    let cwd = std::env::current_dir().expect("Unable to get current working directory");
    let new_dir = cwd.join(&args.template_name);
    if new_dir.exists() {
        println!("Directory already exists");
        return;
    }
    // Create new directory
    fs::create_dir(&new_dir).expect("Unable to create directory");
    // Copy contents of template into directory
    fs_extra::dir::copy(template_dir, cwd, &CopyOptions::new())
        .expect("Unable to copy template");
}
