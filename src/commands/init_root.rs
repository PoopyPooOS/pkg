use libpkg::{PackageManager, error::PackageManagerError};
use logger::info;

pub fn init_root(pm: &mut PackageManager) -> Result<(), PackageManagerError> {
    info!("Initializing rootfs at \"{}\"", pm.root.display());

    pm.init_root()?;

    info!("Done");

    Ok(())
}
