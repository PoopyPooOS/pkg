use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(alias = "i")]
    Install { name: String },
    #[clap(aliases = ["r", "rm"])]
    Remove { name: String },
}
