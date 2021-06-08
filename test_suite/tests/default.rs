extern crate envconfig;

use envconfig::Envconfig;
use std::collections::HashMap;
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

    let config = Config::init_from_env().unwrap();
    assert_eq!(config.port, 5432);
}

#[test]
fn test_when_hashmap_key_is_missing() {
    setup();

    let config = Config::init_from_hashmap(&HashMap::new()).unwrap();
    assert_eq!(config.port, 5432);
}

#[test]
fn test_when_env_is_present() {
    setup();

    env::set_var("PORT", "8080");
    let config = Config::init_from_env().unwrap();
    assert_eq!(config.port, 8080);
}

#[test]
fn test_when_hashmap_key_is_present() {
    setup();

    let mut hashmap = HashMap::new();
    hashmap.insert("PORT".to_string(), "8080".to_string());

    let config = Config::init_from_hashmap(&hashmap).unwrap();
    assert_eq!(config.port, 8080);
}
