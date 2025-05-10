use crate::{
    err,
    error::{Context, PackageManagerError},
    event::Event,
    store::{LOCK_POLL_INTERVAL, check_err, send},
    util::append_to_file,
};
use glob::glob;
use std::{fs, path::PathBuf, sync::mpsc::Sender, thread};

impl crate::PackageManager {
    /// Remove the given package from the symlinks, this must be ran in a separate thread.
    /// This does not remove the package from the store, to do so you need to run the garbage collector.
    /// A mpsc sender must be given to send progress events.
    /// This function requires root privileges.
    pub fn remove<S: Into<String> + Clone>(&self, id: S, version: Option<S>, tx: &Sender<Event>) {
        let result = self.remove_inner(id, version, tx);

        check_err!(tx, self.store_set_immutable(true));
        check_err!(tx, result);
    }

    fn remove_inner<S: Into<String> + Clone>(&self, id: S, version: Option<S>, tx: &Sender<Event>) -> Result<(), PackageManagerError> {
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

        let pattern = &format!(
            "{}/{}-{}",
            self.store().display(),
            id.into(),
            if let Some(version) = version.clone() { version.into() } else { "*".to_string() }
        );
        let packages = glob(pattern).map_err(|_| PackageManagerError::PackageNotInstalled)?.filter_map(Result::ok).collect::<Vec<_>>();

        if packages.is_empty() {
            return err!(PackageNotInstalled);
        }

        for path in &packages {
            if let Some(ver) = version.clone() {
                let ver_path = path.join(ver.into());

                if !ver_path.join("links").exists() {
                    return err!(PackageNotInstalled);
                }

                let links = fs::read_to_string(ver_path.join("links"))
                    .unwrap_or_default()
                    .lines()
                    .filter(|s| !s.trim().is_empty())
                    .map(PathBuf::from)
                    .collect::<Vec<_>>();

                links_to_remove.extend(links);
            } else {
                let versions = fs::read_dir(&path)
                    .context("remove: list versions")?
                    .filter_map(Result::ok)
                    .map(|entry| entry.file_name().display().to_string());

                for version in versions {
                    let links_path = path.join(version).join("links");

                    if !links_path.exists() {
                        continue;
                    }

                    let links = fs::read_to_string(&links_path)
                        .unwrap_or_default()
                        .lines()
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| self.root.join(s))
                        .collect::<Vec<_>>();

                    fs::remove_file(links_path).context("remove: remove links file from package")?;

                    links_to_remove.extend(links);
                }
            }
        }

        append_to_file(self.store().join("garbage"), packages.iter().map(|path| path.display().to_string()).collect::<Vec<String>>().join("\n")).context("remove: append to garbage list in store")?;

        for link in &links_to_remove {
            fs::remove_file(link).context("remove: remove all the symlinks")?;
        }

        Ok(())
    }
}
