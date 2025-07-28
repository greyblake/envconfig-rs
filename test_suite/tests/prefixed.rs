extern crate envconfig;

use envconfig::{Envconfig, Error};
use std::{collections::HashMap, env};

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(prefix = "")]
    pub core: CoreConfig,
    #[envconfig(prefix = "DB1_")]
    pub db1: DatabaseConfig,
    #[envconfig(prefix = "DB2_")]
    pub db2: DatabaseConfig,
}

#[derive(Envconfig)]
pub struct CoreConfig {
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: String,
    pub worker_count: u64,
}

#[derive(Envconfig)]
pub struct DatabaseConfig {
    #[envconfig(from = "DB_HOST")]
    pub host: String,
    #[envconfig(from = "DB_PORT")]
    pub port: u16,
}

fn setup() {
    env::remove_var("DB1_DB_HOST");
    env::remove_var("DB1_DB_PORT");
    env::remove_var("DB2_DB_HOST");
    env::remove_var("DB2_DB_PORT");
    env::remove_var("LOG_LEVEL");
    env::remove_var("WORKER_COUNT");
}

fn set_default_env_values() {
    env::set_var("DB1_DB_HOST", "localhost");
    env::set_var("DB1_DB_PORT", "80");
    env::set_var("DB2_DB_HOST", "127.0.0.1");
    env::set_var("DB2_DB_PORT", "443");
    env::set_var("LOG_LEVEL", "info");
    env::set_var("WORKER_COUNT", "4");
}

fn set_default_hashmap_values() -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    hashmap.insert("DB1_DB_HOST".to_string(), "localhost".to_string());
    hashmap.insert("DB1_DB_PORT".to_string(), "80".to_string());
    hashmap.insert("DB2_DB_HOST".to_string(), "127.0.0.1".to_string());
    hashmap.insert("DB2_DB_PORT".to_string(), "443".to_string());
    hashmap.insert("LOG_LEVEL".to_string(), "info".to_string());
    hashmap.insert("WORKER_COUNT".to_string(), "4".to_string());
    hashmap
}

#[test]
fn test_inits_config_from_env_variables() {
    setup();
    set_default_env_values();

    let config = Config::init_from_env().unwrap();
    assert_eq!(config.core.log_level, "info");
    assert_eq!(config.core.worker_count, 4);
    assert_eq!(config.db1.host, "localhost");
    assert_eq!(config.db1.port, 80);
    assert_eq!(config.db2.host, "127.0.0.1");
    assert_eq!(config.db2.port, 443);
}

#[test]
fn test_inits_config_from_hashmap() {
    setup();

    let hashmap = set_default_hashmap_values();
    let config = Config::init_from_hashmap(&hashmap).unwrap();
    assert_eq!(config.core.log_level, "info");
    assert_eq!(config.core.worker_count, 4);
    assert_eq!(config.db1.host, "localhost");
    assert_eq!(config.db2.host, "127.0.0.1");
}

#[test]
fn test_checks_presence_of_env_vars() {
    setup();

    // We are relying on the order of parsing here to prove that by setting `WORKER_COUNT` we'll get an error on the next field
    env::set_var("WORKER_COUNT", "4");

    let err = Config::init_from_env().err().unwrap();
    let expected_err = Error::EnvVarMissing {
        name: "DB1_DB_HOST",
    };
    assert_eq!(err, expected_err);
}

#[test]
fn test_checks_presence_of_hashmap_keys() {
    setup();

    let mut hashmap = HashMap::new();
    // We are relying on the order of parsing here to prove that by setting `WORKER_COUNT` we'll get an error on the next field
    hashmap.insert("WORKER_COUNT".to_string(), "4".to_string());

    let err = Config::init_from_hashmap(&hashmap).err().unwrap();
    let expected_err = Error::EnvVarMissing {
        name: "DB1_DB_HOST",
    };
    assert_eq!(err, expected_err);
}

#[test]
fn test_fails_if_can_not_parse_db_port_from_env() {
    setup();

    env::set_var("WORKER_COUNT", "4");
    env::set_var("DB1_DB_HOST", "localhost");
    env::set_var("DB1_DB_PORT", "67000");

    let err = Config::init_from_env().err().unwrap();
    let expected_err = Error::ParseError {
        name: "DB1_DB_PORT",
    };
    assert_eq!(err, expected_err);
}

#[test]
fn test_fails_if_can_not_parse_db_port_from_hashmap() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("DB1_DB_HOST".to_string(), "localhost".to_string());
    hashmap.insert("DB1_DB_PORT".to_string(), "67000".to_string());
    hashmap.insert("LOG_LEVEL".to_string(), "info".to_string());
    hashmap.insert("WORKER_COUNT".to_string(), "4".to_string());

    let err = Config::init_from_hashmap(&hashmap).err().unwrap();
    let expected_err = Error::ParseError {
        name: "DB1_DB_PORT",
    };
    assert_eq!(err, expected_err);
}
