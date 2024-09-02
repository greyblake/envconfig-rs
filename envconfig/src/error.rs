//! Errors resulting from calling functions in this crate

use std::{error::Error as StdError, fmt};

/// Represents an error, that may be returned by `fn init_from_env()` of trait `Envconfig`.
#[derive(Debug, PartialEq)]
pub enum Error {
    EnvVarMissing { name: &'static str },
    ParseError { name: &'static str },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EnvVarMissing { name } => {
                write!(f, "Environment variable {} is missing", name)
            }
            Error::ParseError { name } => {
                write!(f, "Failed to parse environment variable {}", name)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
