#![feature(io_error_more, let_chains, macro_metavar_expr)]

use std::{
    env,
    fmt::Debug,
    path::{Path, PathBuf},
};

// Re-exports
pub use tl::Source;

pub mod error;
pub mod event;
pub mod generations;
pub mod package;
mod sandbox;

mod manager;
mod nushell;
mod paths;
mod store;

#[derive(Debug)]
pub struct PackageManager {
    pub root: PathBuf,
    old_cwd: PathBuf,
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new_with_root("/")
    }
}

impl PackageManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    /// Panics if it fails to set the cwd to the provided root.
    pub fn new_with_root(root: impl AsRef<Path>) -> Self {
        let old_cwd = env::current_dir().unwrap_or(root.as_ref().to_path_buf());
        env::set_current_dir(&root).expect("Failed to set current directory to root");

        Self {
            root: root.as_ref().to_path_buf(),
            old_cwd,
        }
    }

    #[must_use]
    pub fn root(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().to_path_buf();
        self
    }
}

impl Drop for PackageManager {
    fn drop(&mut self) {
        env::set_current_dir(&self.old_cwd).expect("Failed to set current directory back to the old working directory");
    }
}
