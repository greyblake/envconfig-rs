extern crate envconfig;

use envconfig::Envconfig;
use std::env;

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
    pub url: String,
}

fn setup() {
    env::remove_var("DB1_DB_HOST");
    env::remove_var("DB2_MY_DB_HOST");
    env::remove_var("LOG_LEVEL");
    env::remove_var("WORKER_COUNT");
}

fn set_default_values() {
    env::set_var("DB1_DB_HOST", "localhost");
    env::set_var("DB2_DB_HOST", "127.0.0.1");
    env::set_var("LOG_LEVEL", "info");
    env::set_var("WORKER_COUNT", "4");
}

#[test]
fn test_inits_config_from_env_variables() {
    setup();
    set_default_values();

    // env::set_var("DB_HOST", "localhost");
    let config = Config::init_from_env().unwrap();
    assert_eq!(config.core.log_level, "info");
    assert_eq!(config.core.worker_count, 4);
    assert_eq!(config.db1.url, "localhost");
    assert_eq!(config.db2.url, "127.0.0.1");
}

// #[test]
// fn test_inits_config_from_env_variables() {
//     setup();

//     env::set_var("DB_HOST", "localhost");
//     env::set_var("DB_PORT", "5432");

//     let config = Config::init_from_env().unwrap();
//     assert_eq!(config.db_host, "localhost");
//     assert_eq!(config.db_port, 5432u16);
// }

// #[test]
// fn test_inits_config_from_hashmap() {
//     setup();

//     let mut hashmap = HashMap::new();
//     hashmap.insert("DB_HOST".to_string(), "localhost".to_string());
//     hashmap.insert("DB_PORT".to_string(), "5432".to_string());

//     let config = Config::init_from_hashmap(&hashmap).unwrap();
//     assert_eq!(config.db_host, "localhost");
//     assert_eq!(config.db_port, 5432u16);
// }

// #[test]
// fn test_checks_presence_of_env_vars() {
//     setup();

//     env::set_var("DB_HOST", "localhost");

//     let err = Config::init_from_env().err().unwrap();
//     let expected_err = Error::EnvVarMissing { name: "DB_PORT" };
//     assert_eq!(err, expected_err);
// }

// #[test]
// fn test_checks_presence_of_hashmap_keys() {
//     setup();

//     let mut hashmap = HashMap::new();
//     hashmap.insert("DB_HOST".to_string(), "localhost".to_string());

//     let err = Config::init_from_hashmap(&hashmap).err().unwrap();
//     let expected_err = Error::EnvVarMissing { name: "DB_PORT" };
//     assert_eq!(err, expected_err);
// }

// #[test]
// fn test_fails_if_can_not_parse_db_port_from_env() {
//     setup();

//     env::set_var("DB_HOST", "localhost");
//     env::set_var("DB_PORT", "67000");

//     let err = Config::init_from_env().err().unwrap();
//     let expected_err = Error::ParseError { name: "DB_PORT" };
//     assert_eq!(err, expected_err);
// }

// #[test]
// fn test_fails_if_can_not_parse_db_port_from_hashmap() {
//     setup();

//     let mut hashmap = HashMap::new();
//     hashmap.insert("DB_HOST".to_string(), "localhost".to_string());
//     hashmap.insert("DB_PORT".to_string(), "67000".to_string());

//     let err = Config::init_from_hashmap(&hashmap).err().unwrap();
//     let expected_err = Error::ParseError { name: "DB_PORT" };
//     assert_eq!(err, expected_err);
// }

// #[test]
// fn test_custom_from_str() {
//     use std::num::ParseIntError;
//     use std::str::FromStr;

//     setup();

//     #[derive(Debug, PartialEq)]
//     struct Point {
//         x: i32,
//         y: i32,
//     }

//     impl FromStr for Point {
//         type Err = ParseIntError;

//         fn from_str(s: &str) -> Result<Self, Self::Err> {
//             let coords: Vec<&str> = s
//                 .trim_matches(|p| p == '(' || p == ')')
//                 .split(',')
//                 .collect();

//             let x_fromstr = coords[0].parse::<i32>()?;
//             let y_fromstr = coords[1].parse::<i32>()?;

//             Ok(Point {
//                 x: x_fromstr,
//                 y: y_fromstr,
//             })
//         }
//     }

//     #[derive(Envconfig)]
//     pub struct Config {
//         #[envconfig(from = "POINT")]
//         point: Point,
//     }

//     env::set_var("POINT", "(1,2)");

//     let err = Config::init_from_env().unwrap();
//     assert_eq!(err.point, Point { x: 1, y: 2 });

//     setup();

//     let mut hashmap = HashMap::new();
//     hashmap.insert("POINT".to_string(), "(1,2)".to_string());

//     let err = Config::init_from_hashmap(&hashmap).unwrap();
//     assert_eq!(err.point, Point { x: 1, y: 2 });
// }

// pub fn load_var_with_prefix<T: Envconfig, Clone, S: ::std::hash::BuildHasher>(
//     prefix: &'static str,
//     hashmap: Option<&HashMap<String, T, S>>,
// ) -> Result<T, Error> {
//     let v = match hashmap {
//         None => {
//             T::init_from_env(prefix).map_err(|_| Error::EnvVarMissing { name: prefix })?;
//             None
//         }
//         Some(hashmap) => hashmap.get(prefix).cloned(),
//     };

//     // match hashmap {
//     //     None => env::var(&prefix).ok(),
//     //     Some(hashmap) => hashmap.get(prefix).cloned(),
//     // }
//     // .ok_or(Error::EnvVarMissing { name: &prefix })
//     // .and_then(|string_value| {
//     //     string_value
//     //         .parse::<T>()
//     //         .map_err(|_| Error::ParseError { name: var_name })
//     // })
//     Ok(todo!())
// }
