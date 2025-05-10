use clap::Parser;
use cli::{Cli, Command};
use libpkg::PackageManager;
use logger::fatal;

mod cli;
mod commands;

fn main() {
    let args = Cli::parse();

    let pm = PackageManager::new_with_root(args.root);

    if args.command.needs_complete_root() && !pm.check_root() {
        fatal!("The root that was passed is corrupted.");
        return;
    }

    macro_rules! command {
        ($($pat:pat => $command:expr),* $(,)?) => {
            match args.command {
                $(
                    $pat => if let Err(err) = $command {
                        fatal!("{err}");
                        return;
                    }
                )*
            }
        }
    }

    command!(
        Command::Install { source } => commands::install(pm, source),
        Command::Remove { id } => commands::remove(pm, id),
        Command::InitRoot => commands::init_root(&pm),
    )
}
