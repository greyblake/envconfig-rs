extern crate envconfig;

use envconfig::{Envconfig, Error};
use std::collections::HashMap;
use std::env;

#[derive(Envconfig)]
pub struct Config {
    pub db_host: String,
    pub db_port: Option<u16>,
}

fn setup() {
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
}

#[test]
fn test_derives_env_variable_names_automatically() {
    setup();

    env::set_var("DB_HOST", "db.example.com");
    env::set_var("DB_PORT", "5050");

    let config = Config::init_from_env().unwrap();
    assert_eq!(config.db_host, "db.example.com");
    assert_eq!(config.db_port, Some(5050));
}

#[test]
fn test_derives_hashmap_keys_automatically() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("DB_HOST".to_string(), "db.example.com".to_string());
    hashmap.insert("DB_PORT".to_string(), "5050".to_string());

    let config = Config::init_from_hashmap(&hashmap).unwrap();
    assert_eq!(config.db_host, "db.example.com");
    assert_eq!(config.db_port, Some(5050));
}

#[test]
fn test_var_is_missing() {
    setup();

    let err = Config::init_from_env().err().unwrap();
    let expected_err = Error::EnvVarMissing { name: "DB_HOST" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_key_is_missing() {
    setup();

    let err = Config::init_from_hashmap(&HashMap::default())
        .err()
        .unwrap();
    let expected_err = Error::EnvVarMissing { name: "DB_HOST" };
    assert_eq!(err, expected_err);
}
