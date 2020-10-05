<p align="center"><img width="200" src="https://raw.githubusercontent.com/greyblake/envconfig-rs/master/logo/envconfig.svg" alt="Envconfig logo"></p>

<p align="center">
<a href="https://travis-ci.org/greyblake/envconfig-rs" rel="nofollow"><img src="https://travis-ci.org/greyblake/envconfig-rs.svg?branch=master" alt="Build Status"></a>
<a href="https://raw.githubusercontent.com/greyblake/envconfig-rs/master/LICENSE" rel="nofollow"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
<a href="https://docs.rs/envconfig" rel="nofollow"><img src="https://docs.rs/envconfig/badge.svg" alt="Documentation"></a>
<p>

<p align="center">Initialize config structure from environment variables in Rust without boilerplate.</p>

## Usage

Let's say you application relies on the following environment variables:

* `DB_HOST`
* `DB_PORT`

And you want to initialize `Config` structure like this one:

```rust,ignore
struct Config {
    host: String,
    port: u16,
}
```

You can achieve this with the following code without boilerplate:

```rust
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,
}

fn main() {
    // Assuming the following environment variables are set
    std::env::set_var("DB_HOST", "127.0.0.1");

    // Initialize config from environment variables or terminate the process.
    let config = Config::init_from_env().unwrap();

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

## License

[MIT](https://github.com/greyblake/envconfig-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [allevo](https://github.com/allevo) Tommaso Allevi - support nested structures
- [hobofan](https://github.com/hobofan) Maximilian Goisser - update dependencies
