mod common;
mod create;
mod list;
use clap::{Parser, Subcommand};
use create::CreateArgs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Easily scaffold projects from templates
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "c")]
    Create(CreateArgs),
    #[command(alias = "l")]
    List,
    #[command(alias = "d")]
    Directory,
}
fn main() {
    let cli = Cli::parse();
    let res = match &cli.command {
        Commands::Create(args) => create::create_template(args),
        Commands::List => list::list_templates(),
        Commands::Directory => {
            println!("{}", common::projection_dir().to_str().unwrap());
            Ok(())
        }
    };
    if let Err(e) = res {
        e.print().expect("Unable to print error");
    };
}
