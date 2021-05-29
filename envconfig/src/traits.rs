use crate::error::Error;
use std::collections::HashMap;

/// Indicates that structure can be initialize from environment variables.
pub trait Envconfig {
    /// Initialize structure from environment variables.
    /// Deprecated in favor of init_from_env().
    #[deprecated(
        since = "0.9.0",
        note = "Function init() is deprecated. Please use init_from_env() instead."
    )]
    fn init() -> Result<Self, Error>
    where
        Self: Sized;

    /// Initialize structure from environment variables.
    fn init_from_env() -> Result<Self, Error>
    where
        Self: Sized;

    /// Initialize structure from a hashmap.
    fn init_from_hashmap(hashmap: HashMap<String, String>) -> Result<Self, Error>
    where
        Self: Sized;
}
