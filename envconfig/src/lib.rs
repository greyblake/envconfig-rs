//! Envconfig is a Rust library that helps to initialize configuration structure
//! from environment variables.
//! It makes use of custom derive macros to reduce boilerplate.
//!
//! Example
//!
//! ```
//! use std::env;
//! use envconfig::Envconfig;
//!
//! #[derive(Envconfig)]
//! struct Config {
//!     #[envconfig(from = "DB_HOST")]
//!     pub db_host: String,
//!
//!     #[envconfig(from = "DB_PORT")]
//!     pub db_port: Option<u16>,
//!
//!     #[envconfig(from = "HTTP_PORT", default = "8080")]
//!     pub http_port: u16,
//! }
//!
//! // We assume that those environment variables are set somewhere outside
//! env::set_var("DB_HOST", "localhost");
//! env::set_var("DB_PORT", "5432");
//!
//! // Initialize config from environment variables
//! let config = Config::init_from_env().unwrap();
//!
//! assert_eq!(config.db_host, "localhost");
//! assert_eq!(config.db_port, Some(5432));
//! assert_eq!(config.http_port, 8080);
//! ```
//!
//! ## Nested Configuration with Prefixes
//!
//! You can also nest configuration structures with prefixes:
//!
//! ```
//! use std::env;
//! use envconfig::Envconfig;
//!
//! #[derive(Envconfig)]
//! struct DbConfig {
//!     #[envconfig(from = "HOST")]  // Will look for DB_HOST
//!     pub host: String,
//!
//!     #[envconfig(from = "PORT")]  // Will look for DB_PORT
//!     pub port: u16,
//! }
//!
//! #[derive(Envconfig)]
//! struct Config {
//!     #[envconfig(nested, prefix = "DB_")]
//!     pub db: DbConfig,
//! }
//!
//! // Set environment variables with prefixes
//! env::set_var("DB_HOST", "localhost");
//! env::set_var("DB_PORT", "5432");
//!
//! // Initialize config from environment variables
//! let config = Config::init_from_env().unwrap();
//!
//! assert_eq!(config.db.host, "localhost");
//! assert_eq!(config.db.port, 5432);
//! ```
//!
//! The library uses `std::str::FromStr` trait to convert environment variables into custom
//! data type. So, if your data type does not implement `std::str::FromStr` the program
//! will not compile.

mod error;
mod traits;
mod utils;

pub use error::Error;
pub use traits::Envconfig;
pub use utils::{load_optional_var, load_var, load_var_with_default};

// re-export derive
pub use envconfig_derive::Envconfig;
