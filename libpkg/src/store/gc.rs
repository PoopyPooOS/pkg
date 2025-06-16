use crate::error::{Context, Result};
use std::{fs, io, path::Path};

impl crate::PackageManager {
    pub fn store_gc(&self) -> Result<()> {
        /// Returns the amount of dead symlinks removed from the given directory.
        fn remove_dead_symlinks(path: impl AsRef<Path>) -> io::Result<usize> {
            let mut count = 0;

            for entry in fs::read_dir(path)?.flatten() {
                let path = entry.path();

                // Check if it's a broken symlink
                if path.symlink_metadata()?.file_type().is_symlink() && fs::metadata(&path).is_err() {
                    fs::remove_file(&path)?;
                    count += 1;
                }
            }

            Ok(count)
        }

        let garbage = fs::read_to_string(self.store().join("garbage"))
            .unwrap_or_default()
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| self.root.join(s))
            .collect::<Vec<_>>();

        for path in garbage {
            if path.join("links").exists() {
                // Invalid entry, links would have been removed if the package should've been garbage collected.
                continue;
            }

            fs::remove_dir_all(&path).context(format!("store_gc: remove path '{}' found in garbage tracker", path.display()))?;
        }

        remove_dead_symlinks(self.root.join("bin")).context("store_gc: remove dead symlinks from /bin")?;
        remove_dead_symlinks(self.root.join("lib")).context("store_gc: remove dead symlinks from /lib")?;

        Ok(())
    }
}
