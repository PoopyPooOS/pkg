use libpkg::PackageManager;
use prelude::logger::info;

use crate::{err, error::Error};

pub fn init_root(pm: &PackageManager) -> Result<(), Error> {
    if pm.check_root() {
        return err!(AlreadyInitialized);
    }

    info!("Initializing rootfs at \"{}\"", pm.root.display());

    pm.init_root()?;

    info!("Done");

    Ok(())
}
