macro_rules! export_cmd {
    ($($name:ident),*) => {
        $(
            mod $name;
            pub use $name::$name;
        )*
    };
}

export_cmd!(install, remove, init_root);
