use logger::Log;
use std::{io, num::ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackageManagerError {
    #[error("The specified package version is not installed")]
    PackageVersionNotInstalled,
    #[error("The package does not exist")]
    PackageNotInstalled,
    #[error("The package is already installed")]
    PackageAlreadyInstalled,
    #[error("Error setting user id")]
    SetUID,
    #[error("Error evaluating package: {0}")]
    PackageEval(Box<Log>),
    #[error("Error Parsing Int: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("I/O Error: {0}")]
    IO(#[from] io::Error),
    #[error("I/O Error: {0}")]
    RustixIO(#[from] rustix::io::Errno),
    #[error("OS Error: {0}")]
    Nix(#[from] nix::errno::Errno),
}

impl From<Box<Log>> for PackageManagerError {
    fn from(value: Box<Log>) -> Self {
        Self::PackageEval(value)
    }
}

#[macro_export]
macro_rules! err {
    ($err:ident) => {
        Err($crate::error::PackageManagerError::$err)
    };
    ($err:ident($data:expr)) => {
        Err($crate::error::PackageManagerError::$err($data))
    };
}
