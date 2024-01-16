mod create;
mod list;
mod common;

use clap::{Parser, Subcommand};
use create::CreateArgs;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Easily scaffold projects from templates
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "c")]
    Create(CreateArgs),
    #[command(alias = "l")]
    List,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Create(args)) => create::create_template(args),
        Some(Commands::List) => list::list_templates(),
        None => {
            println!("No command specified");
        }
    }
}
