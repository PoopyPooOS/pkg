use crate::error::PackageManagerError;

#[derive(Debug)]
pub enum Event {
    /// Waiting for the lock to be removed from the store.
    AwaitingUnlock,
    /// When the store has been unlocked.
    Unlocked,
    /// Creating the directory for the package to be installed in the store.
    AllocatingInStore,
    /// (number of bytes copied, number of bytes to copy in total)
    CopySrcProgress(u64, u64),
    Error(PackageManagerError),
}
