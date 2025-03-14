use crate::error::PackageManagerError;

#[derive(Debug)]
pub enum Event {
    AllocatingStore,
    AwaitingUnlock,
    Unlocked,
    Error(PackageManagerError),
}
