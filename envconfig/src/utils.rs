use std::env;
use std::str::FromStr;

use Error;

/// Load a nenvironment variable by name and parse it into type `T`.
/// The function is used by `envconfig_derive` to implement `init()`.
///
/// It returns `Error` in the following cases:
/// - Environment variable is not present
/// - Parsing failed
pub fn load_var<T: FromStr>(var_name: &'static str) -> Result<T, Error> {
    env::var(var_name)
        .map_err(|_| Error::EnvVarMissing { name: var_name })
        .and_then(|string_value| {
            string_value
                .parse::<T>()
                .map_err(|_| Error::ParseError { name: var_name })
        })
}

pub fn load_optional_var<T: FromStr>(var_name: &'static str) -> Result<Option<T>, Error> {
    let res_var = env::var(var_name);

    match res_var {
        Err(_) => Ok(None),
        Ok(string_value) => string_value
            .parse::<T>()
            .map(|v| Some(v))
            .map_err(|_| Error::ParseError { name: var_name }),
    }
}
