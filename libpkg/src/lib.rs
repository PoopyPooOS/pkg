#![feature(io_error_more, let_chains, if_let_guard, macro_metavar_expr)]

use error::{Context, PackageManagerError};
use nu_embed::Engine;
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

mod manager;
mod paths;
mod store;
mod util;

#[derive(Debug)]
pub struct PackageManager {
    pub root: PathBuf,
    old_cwd: PathBuf,

    nu_engine: nu_embed::Engine,
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

        Self {
            root: root.as_ref().to_path_buf(),
            old_cwd,
            nu_engine: Engine::new(),
        }
    }

    /// Run something with the cwd as the root property.
    /// Useful for things like symlinks which need to be relatie to the root path.
    pub fn with_root_cwd(&self, callback: impl FnOnce() -> Result<(), PackageManagerError>) -> Result<(), PackageManagerError> {
        env::set_current_dir(&self.root).context("with_root_cwd: set the cwd to the specified root")?;
        callback()?;
        env::set_current_dir(&self.old_cwd).context("with_root_cwd: set the cwd to the old cwd")?;

        Ok(())
    }

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
