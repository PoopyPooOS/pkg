use crate::{err, error::PackageManagerError, event::Event, package::Package, paths};
use rustix::fs::{IFlags, ioctl_getflags, ioctl_setflags};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    sync::mpsc::Sender,
    thread,
    time::Duration,
};

const LOCK_POLL_INTERVAL: Duration = Duration::from_secs(1);

macro_rules! send {
    ($tx:expr, $event:ident) => {
        $tx.send(Event::$event).unwrap();
    };
    ($tx:expr, $event:ident($data:expr)) => {
        $tx.send(Event::$event($data)).unwrap();
    };
}

#[derive(Debug)]
pub struct StoreItem {
    pub id: String,
    pub version: String,
    pub links: Vec<PathBuf>,
}

impl super::PackageManager {
    pub fn store_set_immutable(&self, immutable: bool) -> Result<(), PackageManagerError> {
        let store = paths::store(&self.root);
        let store_fd = OpenOptions::new().read(true).write(false).open(store)?;

        let flags = if immutable { IFlags::IMMUTABLE } else { IFlags::empty() };

        ioctl_setflags(store_fd, flags)?;

        Ok(())
    }

    pub fn store_is_immutable(&self) -> Result<bool, PackageManagerError> {
        let store = paths::store(&self.root);
        let store_fd = OpenOptions::new().read(true).write(false).open(store)?;

        let flags = ioctl_getflags(store_fd)?;

        Ok(flags.contains(IFlags::IMMUTABLE))
    }

    /// Check if a given store item exists.
    pub fn get_store_item<S: AsRef<str>>(&self, id: S, version: S) -> Option<StoreItem> {
        let store = paths::store(&self.root);

        let id = id.as_ref();
        let version = version.as_ref();

        let item_path = store.join(id).join(version);

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

    /// Install a given package, this must be ran in a separate thread.
    /// A mpsc sender must be given to send progress events.
    /// This function requires root privileges.
    pub fn install(&self, package: Package, tx: &Sender<Event>) {
        let result = self.install_inner(package, tx);

        if let Err(err) = self.store_set_immutable(true) {
            let _ = tx.send(Event::Error(err));
        }

        if let Err(err) = result {
            let _ = tx.send(Event::Error(err));
        }
    }

    fn install_inner(&self, package: Package, tx: &Sender<Event>) -> Result<(), PackageManagerError> {
        // Check if store is locked
        if !self.store_is_immutable()? {
            send!(tx, AwaitingUnlock);

            while !self.store_is_immutable()? {
                thread::sleep(LOCK_POLL_INTERVAL);
            }

            send!(tx, Unlocked);
        }

        send!(tx, AllocatingStore);

        self.store_set_immutable(false)?;

        let path = paths::store(&self.root).join(package.id).join(package.version);

        if path.exists() {
            return err!(PackageAlreadyInstalled);
        }

        fs::create_dir_all(path)?;

        Ok(())
    }

    /// Install a given package, this must be ran in a separate thread.
    /// A mpsc sender must be given to send progress events.
    /// This function requires root privileges.
    pub fn remove<S: Into<String>>(&self, id: S, version: Option<S>, tx: &Sender<Event>) {
        let result = self.remove_inner(id, version, tx);

        if let Err(err) = self.store_set_immutable(true) {
            let _ = tx.send(Event::Error(err));
        }

        if let Err(err) = result {
            let _ = tx.send(Event::Error(err));
        }
    }

    fn remove_inner<S: Into<String>>(&self, id: S, version: Option<S>, tx: &Sender<Event>) -> Result<(), PackageManagerError> {
        // Check if store is locked
        if !self.store_is_immutable()? {
            send!(tx, AwaitingUnlock);

            while !self.store_is_immutable()? {
                thread::sleep(LOCK_POLL_INTERVAL);
            }

            send!(tx, Unlocked);
        }

        self.store_set_immutable(false)?;

        let mut links_to_remove: Vec<PathBuf> = Vec::new();

        let path = paths::store(&self.root).join(id.into());

        if !path.exists() {
            return err!(PackageNotInstalled);
        }

        if let Some(ver) = version {
            let ver_path = path.join(ver.into());

            if !ver_path.exists() {
                return err!(PackageVersionNotInstalled);
            }

            let links = fs::read_to_string(ver_path.join("links"))?
                .lines()
                .filter(|s| !s.trim().is_empty())
                .map(PathBuf::from)
                .collect::<Vec<_>>();

            links_to_remove.extend(links);
        } else {
            let versions = fs::read_dir(&path)?
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().display().to_string());

            for version in versions {
                let links = fs::read_to_string(path.join(version).join("links"))?
                    .lines()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| self.root.join(s))
                    .collect::<Vec<_>>();

                links_to_remove.extend(links);
            }
        }

        for link in &links_to_remove {
            dbg!(&link);
            fs::remove_file(link)?;
        }

        Ok(())
    }
}
