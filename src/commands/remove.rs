use libpkg::{PackageManager, error::PackageManagerError, event::Event};
use logger::{error, info, trace};
use std::{sync::mpsc, thread};

pub fn remove(pm: PackageManager, id: impl ToString) -> Result<(), PackageManagerError> {
    let id = id.to_string();
    info!("Removing \"{id}\"");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || pm.remove(id, None, &tx));

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
