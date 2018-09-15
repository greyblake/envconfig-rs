use Error;

/// Indicates that structure can be initialize from environment variables.
pub trait Envconfig {
    /// Initialize structure from environment variables.
    fn init() -> Result<Self, Error>
    where
        Self: Sized;
}
