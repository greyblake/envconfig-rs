#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;
use std::env;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT")]
    pub port: u16,
}

#[test]
fn test_when_default_is_not_specified_value_is_loaded_from_env_var() {
    env::set_var("PORT", "8080");

    let config = Config::init().unwrap();
    assert_eq!(config.port, 8080);
}
