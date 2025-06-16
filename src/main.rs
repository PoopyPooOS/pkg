use cli::{Cli, Command};
use error::{err, Error};
use libpkg::PackageManager;

mod cli;
mod commands;

mod error {
    use libpkg::error::PackageManagerError;
    use prelude::logger::Log;

    #[prelude::error_enum]
    pub enum Error {
        #[error("The root that was passed is corrupted.")]
        CorruptedRoot,
        #[error("The root that was given is already initialized.")]
        AlreadyInitialized,

        #[error("{0}")]
        PkgError(#[from] PackageManagerError),
    }

    impl From<Box<Log>> for ErrorKind {
        fn from(value: Box<Log>) -> Self {
            Self::PkgError(PackageManagerError::PackageEval(value))
        }
    }
}

#[prelude::entry(err: Error)]
fn main(args: Cli) {
    let pm = PackageManager::new_with_root(args.root);

    if args.command.needs_complete_root() && !pm.check_root() {
        return err!(CorruptedRoot);
    }

    match args.command {
        Command::Install { source } => commands::install(pm, source),
        Command::Remove { id } => commands::remove(pm, id),
        Command::InitRoot => commands::init_root(&pm),
    }
}
