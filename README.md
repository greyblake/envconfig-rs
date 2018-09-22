# Envconfig

[![Build Status](https://travis-ci.org/greyblake/envconfig-rs.svg?branch=master)](https://travis-ci.org/greyblake/envconfig-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/greyblake/envconfig-rs/master/LICENSE)
[![Documentation](https://docs.rs/envconfig/badge.svg)](https://docs.rs/envconfig)

Build a config structure form environment variables in Rust without boilerplate.

## Usage

Let's say you application relies on the following environment variables:

* `DB_HOST`
* `DB_PORT`

And you want to initialize `Config` structure like this one:

```rust,ignore
struct Config {
  host: String,
  port: u16
}
```

You can achieve this with the following code without boilerplate:

```rust
#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT")]
    pub db_port: u16,
}

fn main() {
    // Assuming the following environment variables are set
    std::env::set_var("DB_HOST", "127.0.0.1");
    std::env::set_var("DB_PORT", "5432");

    // Initialize config from environment variables or terminate the process.
    let config = Config::init().unwrap();

    assert_eq!(config.db_host, "127.0.0.1");
    assert_eq!(config.db_port, 5432);
}
```

## Running tests

Tests do some manipulation with environment variables, so to
prevent flaky tests they have to be executed in a single thread:

```
cargo test -- --test-threads=1
```

## Roadmap

* [x] - migrate to the latest versions of `syn` and `quote`
* [x] - support `Option<T>` ([issue](https://github.com/greyblake/envconfig-rs/issues/10))
* [ ] - support `default` attribute ([issue](https://github.com/greyblake/envconfig-rs/issues/3))
* [ ] - support nested structures?

## License

[MIT](https://github.com/greyblake/envconfig-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
