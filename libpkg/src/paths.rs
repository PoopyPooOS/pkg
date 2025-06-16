/// Paths that are required for a root to be valid.
pub const ROOT: &[&str] = &["store/src", "system/generations", "home/root"];

macro_rules! paths {
    ($(
        $(#[$attr:meta])*
        $name:ident $name_raw:ident $path:expr
    ),* $(,)?) => {
        impl super::PackageManager {
            $(
                $(#[$attr])*
                pub(crate) fn $name(&self) -> std::path::PathBuf {
                    self.root.join($path)
                }

                #[allow(dead_code)]
                $(#[$attr])*
                pub(crate) fn $name_raw(&self) -> std::path::PathBuf {
                    $path.into()
                }
            )*
        }
    };
}

paths!(
    /// Return the path to the system generations relative to the root.
    generations generations_raw "system/generations",
    /// Return the path to the package store relative to the root.
    store store_raw "store",
    /// Return the path to the configs relative to the root.
    config config_raw "config",
);
