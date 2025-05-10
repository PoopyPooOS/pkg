use crate::error::{Context, PackageManagerError};
use nix::unistd::Uid;
use rustix::fs::{IFlags, ioctl_getflags, ioctl_setflags};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    time::Duration,
};

mod gc;
mod install;
mod remove;

/// The interval for checking if the store is locked.
const LOCK_POLL_INTERVAL: Duration = Duration::from_secs(1);
/// The user id to be used in the sandbox.
const SANDBOX_UID: Uid = Uid::from_raw(1000);

macro_rules! send {
    ($tx:expr, $event:ident) => {
        $tx.send(Event::$event).unwrap();
    };
    ($tx:expr, $event:ident($($data:tt)*)) => {
        $tx.send(Event::$event($($data)*)).unwrap();
    };
}

macro_rules! check_err {
    ($tx:expr, $expr:expr) => {
        if let Err(err) = $expr {
            let _ = $tx.send(Event::Error(err));
        }
    };
}

pub(crate) use {check_err, send};

#[derive(Debug)]
pub struct StoreItem {
    pub id: String,
    pub version: String,
    pub links: Vec<PathBuf>,
}

impl super::PackageManager {
    pub fn store_set_immutable(&self, immutable: bool) -> Result<(), PackageManagerError> {
        let store_fd = OpenOptions::new()
            .read(true)
            .write(false)
            .open(self.store())
            .context("store_set_immutable: get a fd to the store with read-only perms")?;

        let flags = if immutable { IFlags::IMMUTABLE } else { IFlags::empty() };

        ioctl_setflags(store_fd, flags).context(format!("store_set_immutable: make the store {}", if immutable { "immutable" } else { "mutable" }))?;

        Ok(())
    }

    pub fn store_is_immutable(&self) -> Result<bool, PackageManagerError> {
        let store_fd = OpenOptions::new()
            .read(true)
            .write(false)
            .open(self.store())
            .context("store_is_immutable: get a fd to the store with read-only perms")?;

        let flags = ioctl_getflags(store_fd).context("store_is_immutable: get the store's flags")?;

        Ok(flags.contains(IFlags::IMMUTABLE))
    }

    /// Check if a given store item exists.
    pub fn get_store_item<S: AsRef<str>>(&self, id: S, version: S) -> Option<StoreItem> {
        let id = id.as_ref();
        let version = version.as_ref();

        let item_path = self.store().join(id).join(version);

        if !item_path.exists() {
            return None;
        }

        let links = fs::read_to_string(item_path.join("links"))
            .ok()?
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from)
            .collect::<Vec<_>>();

        Some(StoreItem {
            id: id.to_owned(),
            version: version.to_owned(),
            links,
        })
    }
}
