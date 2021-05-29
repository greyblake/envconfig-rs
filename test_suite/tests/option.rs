extern crate envconfig;

use envconfig::{Envconfig, Error};
use std::collections::HashMap;
use std::env;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT")]
    pub port: Option<u16>,
}

fn setup() {
    env::remove_var("PORT");
}

#[test]
fn test_env_var_is_missing() {
    setup();

    let config = Config::init_from_env().unwrap();
    assert_eq!(config.port, None);
}

#[test]
fn test_hashmap_key_is_missing() {
    setup();

    let config = Config::init_from_hashmap(&HashMap::new()).unwrap();
    assert_eq!(config.port, None);
}

#[test]
fn test_env_var_is_present() {
    setup();

    env::set_var("PORT", "3030");
    let config = Config::init_from_env().unwrap();
    assert_eq!(config.port, Some(3030));
}

#[test]
fn test_hashmap_key_is_present() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("PORT".to_string(), "3030".to_string());

    let config = Config::init_from_hashmap(&hashmap).unwrap();
    assert_eq!(config.port, Some(3030));
}

#[test]
fn test_env_var_is_invalid() {
    setup();

    env::set_var("PORT", "xyz");
    let err = Config::init_from_env().err().unwrap();
    let expected_err = Error::ParseError { name: "PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_hashmap_key_is_invalid() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("PORT".to_string(), "xyz".to_string());

    let err = Config::init_from_hashmap(&hashmap).err().unwrap();
    let expected_err = Error::ParseError { name: "PORT" };
    assert_eq!(err, expected_err);
}
