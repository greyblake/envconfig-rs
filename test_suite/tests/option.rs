/*
#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;
use std::env;

// type Port = Option<u16>;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT")]
    pub port: Option<u16>,
}

#[test]
fn test_if_env_var_is_not_set_then_initialized_with_none() {
    let config = Config::init().unwrap();
    assert_eq!(config.port, None);
}
*/
