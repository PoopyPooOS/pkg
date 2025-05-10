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
            E::AllocatingInStore => trace!("Creating directory in package store"),
            E::CopySrcProgress(_copied, _total) => {
                // TODO: Render a progress bar
            }

            E::Error(err) => match err {
                Error::PackageNotInstalled => error!("Package not installed"),
                _ => return Err(err),
            },
        }
    }

    Ok(())
}
