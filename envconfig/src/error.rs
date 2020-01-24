use thiserror::Error;

/// Represents an error, that may be returned by `fn init()` of trait `Envconfig`.
#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("Env variable is missing: {name:?}")]
    EnvVarMissing { name: &'static str },
    #[error("Failed to parse env variable: {name:?}")]
    ParseError { name: &'static str },
}
