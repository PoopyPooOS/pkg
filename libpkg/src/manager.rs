use crate::{error::PackageManagerError, paths::ROOT};
use std::{fs, os::unix::fs::symlink, path::PathBuf};

impl super::PackageManager {
    pub fn check_root(&self) -> bool {
        ROOT.iter().all(|path| PathBuf::from(path).exists())
    }

    /// Initialize a rootfs with the base config.
    pub fn init_root(&self) -> Result<(), PackageManagerError> {
        // Create dirs
        for path in ROOT {
            let path = PathBuf::from(path);

            if !path.exists() {
                fs::create_dir_all(&path)?;
            }
        }

        // Create base generation
        fs::create_dir_all(self.generations().join("1/bin"))?;
        fs::create_dir_all(self.generations().join("1/lib"))?;
        fs::create_dir_all(self.generations().join("1/config"))?;

        self.set_current_generation(1)?;

        symlink(self.generations().join("current/lib"), "lib")?;
        symlink(self.generations().join("current/bin"), "bin")?;
        symlink(self.generations().join("current/config"), self.config())?;

        // Write base config
        fs::create_dir(self.config().join("system"))?;
        fs::write(self.config().join("system/env.tl"), include_str!("./base-config/env.tl"))?;
        fs::write(self.config().join("system/services.tl"), include_str!("./base-config/services.tl"))?;
        fs::write(self.config().join("system/users.tl"), include_str!("./base-config/users.tl"))?;

        Ok(())
    }
}
