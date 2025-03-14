use crate::{error::PackageManagerError, paths};
use git2::Repository;
use std::fs;

impl super::PackageManager {
    pub fn check_root(&self) -> bool {
        paths::ROOT.iter().map(|path| self.root.join(path)).all(|path| path.exists()) && Repository::open(paths::store(&self.root)).is_ok()
    }

    /// Initialize a rootfs with the base config.
    pub fn init_root(&mut self) -> Result<(), PackageManagerError> {
        // Create dirs
        for path in paths::ROOT {
            let path = self.root.join(path);

            if !path.exists() {
                fs::create_dir_all(&path)?;
            }
        }

        // Write base config
        fs::write(self.root.join("config/system/env.tl"), include_str!("./base-config/env.tl"))?;
        fs::write(self.root.join("config/system/services.tl"), include_str!("./base-config/services.tl"))?;
        fs::write(self.root.join("config/system/users.tl"), include_str!("./base-config/users.tl"))?;

        // Initialize git repo for state.
        self.state = Some(Repository::init(paths::state(&self.root))?);

        // Create symlinks for state

        Ok(())
    }
}
