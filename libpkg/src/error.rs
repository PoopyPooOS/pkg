use prelude::{
    logger::Log,
    thiserror::{self, Error},
};
use std::{io, num::ParseIntError};

pub type Result<T> = core::result::Result<T, PackageManagerError>;

pub trait Context<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T>;
}

impl<T> Context<T, io::Error> for io::Result<T> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| PackageManagerError::io(context, e))
    }
}

impl<T> Context<T, rustix::io::Errno> for rustix::io::Result<T> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| PackageManagerError::rustix_io(context, e))
    }
}

impl<T> Context<T, nix::errno::Errno> for nix::Result<T> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| PackageManagerError::nix(context, e))
    }
}

impl<T> Context<T, fs_extra::error::Result<T>> for fs_extra::error::Result<T> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| PackageManagerError::fs(context, e))
    }
}

#[derive(Debug, Error)]
pub enum PackageManagerError {
    #[error("The given root path does not exist")]
    RootPathDoesntExist,
    #[error("The package does not exist")]
    PackageNotInstalled,
    #[error("The package is already installed")]
    PackageAlreadyInstalled,
    #[error("The package uses a local source but was fetched from a remote location")]
    LocalPathOnRemotePackage,
    #[error("Error setting user id")]
    SetUID,
    #[error("Error evaluating package: {0}")]
    PackageEval(Box<Log>),

    #[error("Error Parsing Int: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("{context}: {source}")]
    IO {
        context: String,
        #[source]
        source: io::Error,
    },
    #[error("{context}: {source}")]
    RustixIO {
        context: String,
        #[source]
        source: rustix::io::Errno,
    },
    #[error("{context}: {source}")]
    Nix {
        context: String,
        #[source]
        source: nix::errno::Errno,
    },
    #[error("{context}: {source}")]
    FS {
        context: String,
        #[source]
        source: fs_extra::error::Error,
    },
}

impl PackageManagerError {
    pub fn io(context: impl Into<String>, err: io::Error) -> Self {
        Self::IO { context: context.into(), source: err }
    }

    pub fn rustix_io(context: impl Into<String>, err: rustix::io::Errno) -> Self {
        Self::RustixIO { context: context.into(), source: err }
    }

    pub fn nix(context: impl Into<String>, err: nix::errno::Errno) -> Self {
        Self::Nix { context: context.into(), source: err }
    }

    pub fn fs(context: impl Into<String>, err: fs_extra::error::Error) -> Self {
        Self::FS { context: context.into(), source: err }
    }
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
    ($err:ident { context: $ctx:expr, source: $src:expr }) => {
        Err($crate::error::PackageManagerError::$err { context: $ctx.into(), source: $src })
    };
    ($err:ident { source: $src:expr, context: $ctx:expr }) => {
        Err($crate::error::PackageManagerError::$err { context: $ctx.into(), source: $src })
    };
}
