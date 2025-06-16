#![allow(unused_imports)]

use crate::{cli::InstallSource, error::Error};
use libpkg::{
    error::{Context, PackageManagerError},
    event::Event,
    package::Package,
    PackageManager, Source,
};
use prelude::logger::{error, info, trace};
use std::{sync::mpsc, thread};

pub fn install(_pm: PackageManager, source: InstallSource) -> Result<(), Error> {
    let package = match source {
        InstallSource::Name(_) => unimplemented!("fetch packages from repositories"),
        InstallSource::Path(path) => Package::eval(Source::from_path(path).context("pkg: read from given install path")?)?,
    };

    dbg!(package);

    // info!("Installing package \"{}\"", package.name);

    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || pm.install(package, &tx));

    // while let Ok(event) = rx.recv() {
    //     use Event as E;
    //     use PackageManagerError as PkgError;

    //     match event {
    //         E::AwaitingUnlock => info!("The package store is locked because of other processes using it"),
    //         E::Unlocked => info!("Package store unlocked"),
    //         E::AllocatingInStore => trace!("Creating directory in package store"),
    //         E::CopySrcProgress(_copied, _total) => {
    //             // TODO: Render a progress bar
    //         }

    //         E::Error(err) => match err {
    //             PkgError::PackageAlreadyInstalled => error!("Package already installed"),
    //             _ => return Err(err.into()),
    //         },
    //     }
    // }

    Ok(())
}
