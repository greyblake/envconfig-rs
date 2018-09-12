use std::str::FromStr;
use std::env;

use Error;

pub fn load_var<T: FromStr>(var_name: &'static str) -> Result<T, Error> {
    env::var(var_name)
        .map_err(|_| Error::EnvVarMissing { name: var_name  })
        .and_then(|string_value| {
            string_value
                .parse::<T>()
                .map_err(|_| Error::ParseError { name: var_name })
        })
}
