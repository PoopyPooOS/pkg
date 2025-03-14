use std::path::{Path, PathBuf};

/// Paths that are required for a root to be valid.
pub const ROOT: &[&str] = &["config/system", "home/root", "store", "system/state"];

pub fn store(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("store")
}

pub fn state(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("system/state")
}
