#![feature(io_error_more, let_chains)]

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use error::PackageManagerError;
use git2::Repository;
// Re-exports
pub use tl::Source;

pub mod error;
pub mod event;
pub mod package;
pub mod state;

mod manager;
mod paths;
mod store;

pub struct PackageManager {
    pub root: PathBuf,
    pub state: Option<Repository>,
}

impl Debug for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackageManager")
            .field("root", &self.root)
            .field("state", &"<git repository>")
            .finish()
    }
}

impl PackageManager {
    pub fn new() -> Result<Self, PackageManagerError> {
        Self::new_with_root("/")
    }

    pub fn new_with_root(root: impl AsRef<Path>) -> Result<Self, PackageManagerError> {
        let state = Repository::open(paths::state(&root)).ok();

        Ok(Self {
            root: root.as_ref().to_path_buf(),
            state,
        })
    }

    #[must_use]
    pub fn root(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().to_path_buf();
        self
    }
}
