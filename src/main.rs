use clap::Parser;
use fs_extra::dir::CopyOptions;
use simple_home_dir;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Easily scaffold projects from templates
struct Args {
    template_name: String,
    destination: Option<String>,
}

fn main() {
    let args = Args::parse();
    // Get all directories
    let dirs: Vec<PathBuf> = fs::read_dir(
        simple_home_dir::home_dir()
            .expect("Cannot find home dir")
            .join("projection"),
    )
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
    let dir_name = {
        if let Some(dest) = &args.destination {
            dest
        } else {
            &args.template_name
        }
    };
    let new_dir = cwd.join(&dir_name);
    if new_dir.exists() {
        let files = new_dir
            .read_dir()
            .expect("Unable to access directory")
            .count();
        if files > 0 {
            panic!("Directory '{}' is non-empty", &dir_name);
        }
    } else {
        // Create new directory
        fs::create_dir(&new_dir).expect("Unable to create directory");
    }
    // Copy contents of template into directory
    fs_extra::dir::copy(
        template_dir,
        new_dir,
        &CopyOptions {
            content_only: true,
            ..Default::default()
        },
    )
    .expect("Unable to copy template");
}
