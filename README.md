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

```rust
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

#[derive(Envconfig)]
pub struct Config {
    #[from="DB_HOST"]
    pub db_host: String,

    #[from="DB_PORT"]
    pub db_port: u16
}

// Initialize config from environment variables or terminate the process.
let config = Self::init().unwrap_or_else(|err| {
    eprintln!("{}", err);
    ::std::process::exit(1);
});
```

## Running tests

To prevent flaky tests run them in one thread:

```
cargo test -p envconfig -- --test-threads=1
```

## License

[MIT](https://github.com/greyblake/envconfig-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
