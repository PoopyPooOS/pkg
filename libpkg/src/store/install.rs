use crate::{
    err,
    error::{Context, PackageManagerError},
    event::Event,
    package::{Package, Src},
    store::{LOCK_POLL_INTERVAL, SANDBOX_UID, check_err, send},
};
use fs_extra::dir::{CopyOptions, TransitProcessResult};
use logger::info;
use nix::{
    sys::wait::waitpid,
    unistd::{ForkResult, chroot, fork, setuid},
};
use std::{
    env,
    fs::{self, File},
    path::Path,
    process,
    sync::mpsc::Sender,
    thread,
};

impl crate::PackageManager {
    /// Install a given package, this must be ran in a separate thread.
    /// A mpsc sender must be given to send progress events.
    /// This function requires root privileges.
    pub fn install(&self, package: Package, tx: &Sender<Event>) {
        let result = self.install_inner(package, tx);

        check_err!(tx, self.store_set_immutable(true));
        check_err!(tx, result);
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

        send!(tx, AllocatingInStore);

        self.store_set_immutable(false)?;

        let package_full_id = format!("{}-{}", package.id, package.version);
        let path = self.store().join(&package_full_id);

        if path.exists() {
            return err!(PackageAlreadyInstalled);
        }

        // Initialize the build environment.
        fs::create_dir_all(&path).context("install: create directory for the build env of the package")?;
        fs::create_dir_all(path.join("bin")).context("install: create bin directory for the build env of the package")?;
        fs::create_dir_all(path.join("lib")).context("install: create lib directory for the build env of the package")?;
        File::create(path.join("links")).context("install: create empty links file for the package")?;

        // Find the path to the `src` field.
        let src = match package.src {
            Src::Path(src) => match src {
                _ if src.is_absolute() => src,
                _ if let Some(package_path) = package.package_path => {
                    let joined = package_path.join(src);
                    joined.canonicalize().unwrap_or(joined)
                }
                _ => return err!(LocalPathOnRemotePackage),
            },
            Src::Git(_) => unimplemented!("fetch source from git repository"),
        };

        // Copy the source
        let prefix = Path::new("/store/src");
        if !(src.starts_with(prefix) && src.components().count() > prefix.components().count()) {
            fs_extra::dir::copy_with_progress(&src, self.store().join("src").join(&package_full_id), &CopyOptions::new().overwrite(true), |progress| {
                send!(tx, CopySrcProgress(progress.copied_bytes, progress.total_bytes));
                TransitProcessResult::OverwriteAll
            })
            .context("install: copy source of package to store")?;
        }

        let nu_engine = self.nu_engine.clone();

        match unsafe { fork().context("install: fork process")? } {
            ForkResult::Parent { child } => {
                waitpid(child, None).context("install: wait for fork to exit")?;
            }
            ForkResult::Child => {
                chroot(&path).context("install: enter sandbox")?;
                env::set_current_dir("/").context("install: set up sandbox")?;

                setuid(SANDBOX_UID).map_err(|_| PackageManagerError::SetUID)?;

                info!("In sandbox: {src:?}");

                nu_engine.eval("touch /hello; ls /");

                process::exit(0);
            }
        }

        Ok(())
    }
}
