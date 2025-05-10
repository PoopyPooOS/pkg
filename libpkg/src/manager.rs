use crate::{
    error::{Context, PackageManagerError},
    paths::ROOT,
};
use std::{
    fs::{self, File},
    os::unix::fs::symlink,
};

impl super::PackageManager {
    pub fn check_root(&self) -> bool {
        ROOT.iter().all(|path| self.root.join(path).exists())
    }

    /// Initialize a rootfs with the base config.
    pub fn init_root(&self) -> Result<(), PackageManagerError> {
        if !self.root.exists() {
            return Err(PackageManagerError::RootPathDoesntExist);
        }

        // Create dirs
        for path in ROOT {
            let path = self.root.join(path);

            if !path.exists() {
                fs::create_dir_all(&path).context("init_root: create all the directories")?;
            }
        }

        File::create(self.store().join("garbage")).context("init_root: create garbage tracker")?;

        // Create base generation
        fs::create_dir_all(self.generations().join("1/bin")).context("init_root: create base generation bin directory")?;
        fs::create_dir_all(self.generations().join("1/lib")).context("init_root: create base generation lib directory")?;
        fs::create_dir_all(self.generations().join("1/config")).context("init_root: create base generation config directory")?;

        self.set_current_generation(1)?;

        self.with_root_cwd(|| {
            symlink(self.generations_raw().join("current/bin"), "bin").context("init_root: symlink current generation 'bin' to '/bin'")?;
            symlink(self.generations_raw().join("current/lib"), "lib").context("init_root: symlink current generation 'lib' to '/lib'")?;
            symlink(self.generations_raw().join("current/config"), self.config_raw()).context("init_root: symlink current generation 'config' to '/config'")?;

            Ok(())
        })?;

        // Write base config
        fs::create_dir(self.config().join("system")).context("init_root: create system config directory")?;
        fs::write(self.config().join("system/env.tl"), include_str!("./base-config/env.tl")).context("init_root: copy base environment config")?;
        fs::write(self.config().join("system/services.tl"), include_str!("./base-config/services.tl")).context("init_root: copy base services config")?;
        fs::write(self.config().join("system/users.tl"), include_str!("./base-config/users.tl")).context("init_root: copy base users config")?;

        self.store_set_immutable(true)?;

        Ok(())
    }
}
