use std::env;
use std::str::FromStr;

use crate::error::Error;
use std::collections::HashMap;

/// Load an environment variable by name and parse it into type `T`.
///
/// This function can also use a hashmap as a fallback or for testing purposes.
///
/// # Errors
/// - Environment variable is not present
/// - Parsing failed
pub fn load_var<T: FromStr, S: ::std::hash::BuildHasher>(
    var_name: &'static str,
    hashmap: Option<&HashMap<String, String, S>>,
) -> Result<T, Error> {
    match hashmap {
        None => env::var(var_name).ok(),
        Some(hashmap) => hashmap.get(var_name).map(std::string::ToString::to_string),
    }
    .ok_or(Error::EnvVarMissing { name: var_name })
    .and_then(|string_value| {
        string_value
            .parse::<T>()
            .map_err(|_| Error::ParseError { name: var_name })
    })
}

/// Tries to load an environment variable by name and parse it into type `T`.
/// If the environment variable is not present, it returns a default value.
///
/// This function can also use a hashmap as a fallback or for testing purposes.
///
/// # Errors
/// - Parsing failed
pub fn load_var_with_default<T: FromStr, S: ::std::hash::BuildHasher>(
    var_name: &'static str,
    hashmap: Option<&HashMap<String, String, S>>,
    default: &'static str,
) -> Result<T, Error> {
    let opt_var = match hashmap {
        None => env::var(var_name).ok(),
        Some(hashmap) => hashmap.get(var_name).map(std::string::ToString::to_string),
    };

    let string_value = match opt_var {
        None => default,
        Some(ref value) => value,
    };

    string_value
        .parse::<T>()
        .map_err(|_| Error::ParseError { name: var_name })
}

/// Tries to load an environment variable by name and parse it into type `T`.
/// If the environment variable is not present, it returns `None`.
///
/// This function can also use a hashmap as a fallback or for testing purposes.
///
/// # Errors
/// - Parsing failed
pub fn load_optional_var<T: FromStr, S: ::std::hash::BuildHasher>(
    var_name: &'static str,
    hashmap: Option<&HashMap<String, String, S>>,
) -> Result<Option<T>, Error> {
    let opt_var = match hashmap {
        None => env::var(var_name).ok(),
        Some(hashmap) => hashmap.get(var_name).map(std::string::ToString::to_string),
    };

    match opt_var {
        None => Ok(None),
        Some(string_value) => string_value
            .parse::<T>()
            .map(Some)
            .map_err(|_| Error::ParseError { name: var_name }),
    }
}
