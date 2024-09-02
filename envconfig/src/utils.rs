use std::env;
use std::str::FromStr;

use crate::error::Error;
use std::collections::HashMap;

/// Load an environment variable by name and parse it into type `T`.
/// The function is used by `envconfig_derive` to implement `init()`.
///
/// It returns `Error` in the following cases:
/// - Environment variable is not present
/// - Parsing failed
pub fn load_var<T: FromStr>(
    var_name: &'static str,
    hashmap: Option<&HashMap<String, String>>,
) -> Result<T, Error> {
    match hashmap {
        None => env::var(var_name).ok(),
        Some(hashmap) => hashmap.get(var_name).map(|val| val.to_string()),
    }
    .ok_or(Error::EnvVarMissing { name: var_name })
    .and_then(|string_value| {
        string_value
            .parse::<T>()
            .map_err(|_| Error::ParseError { name: var_name })
    })
}

pub fn load_var_with_default<T: FromStr>(
    var_name: &'static str,
    hashmap: Option<&HashMap<String, String>>,
    default: &'static str,
) -> Result<T, Error> {
    let opt_var = match hashmap {
        None => env::var(var_name).ok(),
        Some(hashmap) => hashmap.get(var_name).map(|val| val.to_string()),
    };

    let string_value = match opt_var {
        None => default,
        Some(ref value) => value,
    };

    string_value
        .parse::<T>()
        .map_err(|_| Error::ParseError { name: var_name })
}

pub fn load_optional_var<T: FromStr>(
    var_name: &'static str,
    hashmap: Option<&HashMap<String, String>>,
) -> Result<Option<T>, Error> {
    let opt_var = match hashmap {
        None => env::var(var_name).ok(),
        Some(hashmap) => hashmap.get(var_name).map(|val| val.to_string()),
    };

    match opt_var {
        None => Ok(None),
        Some(string_value) => string_value
            .parse::<T>()
            .map(Some)
            .map_err(|_| Error::ParseError { name: var_name }),
    }
}
