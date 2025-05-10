use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

/// Append to a string list file
pub fn append_to_file(path: impl AsRef<Path>, to_append: impl Into<String>) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", to_append.into())?;
    Ok(())
}
