use Error;

pub trait Envconfig {
    fn init() -> Result<Self, Error>
    where
        Self: Sized;

    fn init_or_die() -> Self
    where
        Self: Sized,
    {
        match Self::init() {
            Ok(config) => config,
            Err(err) => {
                eprintln!("{}", err);
                ::std::process::exit(1);
            }
        }
    }
}
