#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;
use std::env;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT", default = "5432")]
    pub port: u16,
}

fn setup() {
    env::remove_var("PORT");
}

#[test]
fn test_when_env_is_missing() {
    setup();

    let config = Config::init().unwrap();
    assert_eq!(config.port, 5432);
}

#[test]
fn test_when_env_is_present() {
    setup();

    env::set_var("PORT", "8080");
    let config = Config::init().unwrap();
    assert_eq!(config.port, 8080);
}
