use clap::Parser;
use cli::{Cli, Command};
use libpkg::{PackageManager, error::PackageManagerError};
use logger::fatal;
use std::io;

mod cli;
mod commands;

fn main() -> Result<(), PackageManagerError> {
    let args = Cli::parse();

    let mut pm = PackageManager::new_with_root(args.root)?;

    if args.command.needs_complete_root() && !pm.check_root() {
        fatal!("The root that was passed is corrupted.");
        return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
    }

    match args.command {
        Command::Install { source } => commands::install(pm, source)?,
        Command::Remove { id } => commands::remove(pm, id)?,
        Command::InitRoot => commands::init_root(&mut pm)?,
    }

    Ok(())
}
