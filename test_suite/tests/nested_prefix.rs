extern crate envconfig;

use envconfig::{Envconfig, Error};
use std::collections::HashMap;
use std::env;

#[derive(Envconfig)]
pub struct DBConfig {
    #[envconfig(from = "HOST")]
    pub host: String,
    #[envconfig(from = "PORT")]
    pub port: u16,
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested, prefix = "DB_")]
    pub db: DBConfig,
    
    #[envconfig(nested, prefix = "CACHE_")]
    pub cache: CacheConfig,
}

#[derive(Envconfig)]
pub struct CacheConfig {
    #[envconfig(from = "HOST")]
    pub host: String,
    
    #[envconfig(from = "PORT", default = "6379")]
    pub port: u16,
}

#[derive(Envconfig)]
pub struct MultiConfig {
    #[envconfig(nested, prefix = "DB1_")]
    pub db1: DBConfig,

    #[envconfig(nested, prefix = "DB2_")]
    pub db2: DBConfig,
}

fn setup() {
    // Clean up all environment variables we might use in tests
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
    env::remove_var("CACHE_HOST");
    env::remove_var("CACHE_PORT");
    env::remove_var("DB1_HOST");
    env::remove_var("DB1_PORT");
    env::remove_var("DB2_HOST");
    env::remove_var("DB2_PORT");
}

#[test]
fn test_nested_prefix_env() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");
    env::set_var("CACHE_HOST", "redis");
    // CACHE_PORT uses default value

    let config = Config::init_from_env().unwrap();
    assert_eq!(config.db.host, "localhost");
    assert_eq!(config.db.port, 5432u16);
    assert_eq!(config.cache.host, "redis");
    assert_eq!(config.cache.port, 6379u16);
}

#[test]
fn test_nested_prefix_hashmap() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("DB_HOST".to_string(), "localhost".to_string());
    hashmap.insert("DB_PORT".to_string(), "5432".to_string());
    hashmap.insert("CACHE_HOST".to_string(), "redis".to_string());
    // CACHE_PORT uses default value

    let config = Config::init_from_hashmap(&hashmap).unwrap();
    assert_eq!(config.db.host, "localhost");
    assert_eq!(config.db.port, 5432u16);
    assert_eq!(config.cache.host, "redis");
    assert_eq!(config.cache.port, 6379u16);
}

#[test]
fn test_nested_prefix_env_error() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("CACHE_HOST", "redis");
    // DB_PORT is missing

    let err = Config::init_from_env().err().unwrap();
    let expected_err = Error::EnvVarMissing { name: "PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_nested_prefix_hashmap_error() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("DB_HOST".to_string(), "localhost".to_string());
    hashmap.insert("CACHE_HOST".to_string(), "redis".to_string());
    // DB_PORT is missing

    let err = Config::init_from_hashmap(&hashmap).err().unwrap();
    let expected_err = Error::EnvVarMissing { name: "PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_multiple_nested_prefix_env() {
    setup();

    env::set_var("DB1_HOST", "primary");
    env::set_var("DB1_PORT", "5432");
    env::set_var("DB2_HOST", "replica");
    env::set_var("DB2_PORT", "5433");

    let config = MultiConfig::init_from_env().unwrap();
    assert_eq!(config.db1.host, "primary");
    assert_eq!(config.db1.port, 5432u16);
    assert_eq!(config.db2.host, "replica");
    assert_eq!(config.db2.port, 5433u16);
}

#[test]
fn test_multiple_nested_prefix_hashmap() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("DB1_HOST".to_string(), "primary".to_string());
    hashmap.insert("DB1_PORT".to_string(), "5432".to_string());
    hashmap.insert("DB2_HOST".to_string(), "replica".to_string());
    hashmap.insert("DB2_PORT".to_string(), "5433".to_string());

    let config = MultiConfig::init_from_hashmap(&hashmap).unwrap();
    assert_eq!(config.db1.host, "primary");
    assert_eq!(config.db1.port, 5432u16);
    assert_eq!(config.db2.host, "replica");
    assert_eq!(config.db2.port, 5433u16);
}