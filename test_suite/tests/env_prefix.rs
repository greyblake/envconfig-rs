extern crate envconfig;

use envconfig::{Envconfig};
use std::env;

#[derive(Envconfig)]
#[envconfig(env_prefix = "TEST_")]
pub struct ConfigWithoutFrom {
    pub db_host: String,
    pub db_port: u16,
}

fn setup() {
    env::remove_var("TEST_DB_HOST");
    env::remove_var("TEST_DB_PORT");
}

#[test]
fn test_init_from_env_with_env_prefix_and_no_envconfig_on_attributes() {
    setup();

    env::set_var("TEST_DB_HOST", "localhost");
    env::set_var("TEST_DB_PORT", "5432");


    let config = ConfigWithoutFrom::init_from_env().unwrap();
    assert_eq!(config.db_host, "localhost");
    assert_eq!(config.db_port, 5432u16);
}

#[derive(Envconfig)]
#[envconfig(env_prefix = "TEST_")]
pub struct ConfigWithFrom {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT")]
    pub db_port: u16,
}

#[test]
fn test_init_from_env_with_env_prefix_and_envconfig_on_attributes() {
    setup();

    env::set_var("TEST_DB_HOST", "localhost");
    env::set_var("TEST_DB_PORT", "5433");


    let config = ConfigWithFrom::init_from_env().unwrap();
    assert_eq!(config.db_host, "localhost");
    assert_eq!(config.db_port, 5433u16);
}
