extern crate envconfig;

use envconfig::{Envconfig, Error};
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
fn test_var_is_missing() {
    setup();

    let config = Config::init().unwrap();
    assert_eq!(config.port, None);
}

#[test]
fn test_var_is_present() {
    setup();

    env::set_var("PORT", "3030");
    let config = Config::init().unwrap();
    assert_eq!(config.port, Some(3030));
}

#[test]
fn test_var_is_invalid() {
    setup();

    env::set_var("PORT", "xyz");
    let err = Config::init().err().unwrap();
    let expected_err = Error::ParseError { name: "PORT" };
    assert_eq!(err, expected_err);
}
