use Error;

/// Indicates that structure can be initialize from environment variables.
pub trait Envconfig {
    /// Initialize structure from environment variables.
    fn init() -> Result<Self, Error>
    where
        Self: Sized;

    /// Initialize structure from environment variables. If it fails, then
    /// terminate a process with a meaningful error message.
    fn init_or_die() -> Self
    where
        Self: Sized,
    {
        Self::init().unwrap_or_else(|err| {
            eprintln!("{}", err);
            ::std::process::exit(1);
        })
    }
}
