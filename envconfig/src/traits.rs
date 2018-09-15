use Error;

/// Indicates that structure can be initialize from environment variables.
pub trait Envconfig {
    fn init() -> Result<Self, Error>
    where
        Self: Sized;

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
