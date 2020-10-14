extern crate envconfig;

use envconfig::{Envconfig, Error};
use std::env;

#[derive(Envconfig)]
pub struct DBConfig {
    #[envconfig(from = "DB_HOST")]
    pub host: String,

    #[envconfig(from = "DB_PORT")]
    pub port: u16,
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested = true)]
    pub db: DBConfig,
}

#[derive(Envconfig)]
pub struct ConfigDouble {
    #[envconfig(nested = true)]
    pub db1: DBConfig,

    #[envconfig(nested = true)]
    pub db2: DBConfig,
}

fn setup() {
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
}

#[test]
fn test_neasting() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = Config::init_from_env().unwrap();
    assert_eq!(config.db.host, "localhost");
    assert_eq!(config.db.port, 5432u16);
}

#[test]
fn test_neasting_error() {
    setup();

    env::set_var("DB_HOST", "localhost");

    let err = Config::init_from_env().err().unwrap();
    let expected_err = Error::EnvVarMissing { name: "DB_PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_duplicated_are_allowed() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = ConfigDouble::init_from_env().unwrap();
    assert_eq!(config.db1.host, "localhost");
    assert_eq!(config.db1.port, 5432u16);
    assert_eq!(config.db2.host, "localhost");
    assert_eq!(config.db2.port, 5432u16);
}
