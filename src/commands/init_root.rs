use libpkg::{PackageManager, error::PackageManagerError};
use logger::{error, info};

pub fn init_root(pm: &PackageManager) -> Result<(), PackageManagerError> {
    if pm.check_root() {
        error!("The root that was given is already initialized.");
        return Ok(());
    }

    info!("Initializing rootfs at \"{}\"", pm.root.display());

    pm.init_root()?;

    info!("Done");

    Ok(())
}
