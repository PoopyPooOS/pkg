use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, default_value = "/")]
    pub root: PathBuf,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    #[clap(alias = "i")]
    Install {
        #[clap(value_parser = parse_install_source)]
        source: InstallSource,
    },
    #[clap(aliases = ["r", "rm"])]
    Remove { id: String },
    #[clap(alias = "init")]
    InitRoot,
}

impl Command {
    pub fn needs_complete_root(&self) -> bool {
        !matches!(self, Self::InitRoot { .. })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum InstallSource {
    Name(String),
    Path(PathBuf),
}

#[allow(clippy::unnecessary_wraps, reason = "clap requires a Result to be returned")]
fn parse_install_source(input: &str) -> Result<InstallSource, String> {
    let path = PathBuf::from(input);

    if path.exists() && path.is_file() {
        Ok(InstallSource::Path(path))
    } else {
        Ok(InstallSource::Name(input.to_string()))
    }
}
