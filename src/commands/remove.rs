use libpkg::{error::PackageManagerError, event::Event, PackageManager};
use prelude::logger::{error, info, trace};
use std::{sync::mpsc, thread};

use crate::error::Error;

pub fn remove(pm: PackageManager, id: impl ToString) -> Result<(), Error> {
    let id = id.to_string();
    info!("Removing \"{id}\"");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || pm.remove(id, None, &tx));

    while let Ok(event) = rx.recv() {
        use Event as E;
        use PackageManagerError as PkgError;

        match event {
            E::AwaitingUnlock => info!("The package store is locked because of other processes using it"),
            E::Unlocked => info!("Package store unlocked"),
            E::AllocatingInStore => trace!("Creating directory in package store"),
            E::CopySrcProgress(_copied, _total) => {
                // TODO: Render a progress bar
            }

            E::Error(err) => match err {
                PkgError::PackageNotInstalled => error!("Package not installed"),
                _ => return Err(err.into()),
            },
        }
    }

    Ok(())
}
