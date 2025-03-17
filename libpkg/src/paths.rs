/// Paths that are required for a root to be valid.
pub const ROOT: &[&str] = &["store", "system/generations", "home/root"];

macro_rules! paths {
    ($(
        $(#[$attr:meta])*
        $name:ident, $path:expr
    ),*) => {
        impl super::PackageManager {
            $(
                $(#[$attr])*
                pub(crate) fn $name(&self) -> std::path::PathBuf {
                    $path.into()
                }
            )*
        }
    };
}

#[rustfmt::skip]
paths!(
    /// Return the path to the system generations relative to the root.
    generations, "system/generations",
    /// Return the path to the package store relative to the root.
    store, "store",
    /// Return the path to the configs relative to the root.
    config, "config"
);
