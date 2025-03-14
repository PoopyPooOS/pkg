use crate::cli::InstallSource;
use libpkg::{PackageManager, Source, error::PackageManagerError, event::Event, package::Package};
use logger::{error, info, trace};
use std::{sync::mpsc, thread};

pub fn install(pm: PackageManager, source: InstallSource) -> Result<(), PackageManagerError> {
    let package = match source {
        InstallSource::Name(_) => unimplemented!("fetch packages from repositories"),
        InstallSource::Path(path) => Package::eval(Source::from_path(path)?)?,
    };

    info!("Installing package \"{}\"", package.name);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || pm.install(package, &tx));

    while let Ok(event) = rx.recv() {
        use Event as E;
        use PackageManagerError as Error;

        match event {
            E::AwaitingUnlock => info!("The package store is locked because of other processes using it"),
            E::Unlocked => info!("Package store unlocked"),
            E::AllocatingStore => trace!("Creating directory in package store"),

            E::Error(err) => match err {
                Error::PackageAlreadyInstalled => error!("Package already installed"),
                _ => return Err(err),
            },
        }
    }

    Ok(())
}
