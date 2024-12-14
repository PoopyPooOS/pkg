use clap::Parser;
use cli::{Cli, Command};
use logger::info;

mod cli;

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Install { name } => info!(format!("Installing '{name}'")),
        Command::Remove { name } => info!(format!("Removing '{name}'")),
    }
}
