use crate::common::{projection_dir, template_dirs};
use clap::Parser;
use fs_extra::dir::CopyOptions;
use std::fs;

#[derive(Parser, Debug)]
pub struct CreateArgs {
    template_name: String,
    destination: Option<String>,
}
pub fn create_template(args: &CreateArgs) {
    let projection_dir = projection_dir();
    if !projection_dir.exists() {
        fs::create_dir(&projection_dir).expect("Couldn't create projection dir");
    }
    // Get all directories
    let dirs = template_dirs(projection_dir);

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
    let new_dir = cwd.join(dir_name);
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
