<p align="center"><img width="200" src="https://raw.githubusercontent.com/greyblake/envconfig-rs/master/logo/envconfig.svg" alt="Envconfig logo"></p>

<p align="center">
<a href="https://travis-ci.org/greyblake/envconfig-rs" rel="nofollow"><img src="https://travis-ci.org/greyblake/envconfig-rs.svg?branch=master" alt="Build Status"></a>
<a href="https://raw.githubusercontent.com/greyblake/envconfig-rs/master/LICENSE" rel="nofollow"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
<a href="https://docs.rs/envconfig" rel="nofollow"><img src="https://docs.rs/envconfig/badge.svg" alt="Documentation"></a>
<p>

<p align="center">Initialize config structure from environment variables in Rust without boilerplate.</p>

## Install

```rust
[dependencies]
envconfig = "0.10.0"
```

## Usage

### Basic example

Let's say you application relies on the following environment variables:

* `DB_HOST`
* `DB_PORT`

And you want to initialize `Config` structure like this one:

```rust,ignore
struct Config {
    db_host: String,
    db_port: u16,
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

### Nested configs

Configs can be nested. Just add `#[envconfig(nested = true)]` to nested field.

```rust
#[derive(Envconfig)]
pub struct DbConfig {
    #[envconfig(from = "DB_HOST")]
    pub host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub port: u16,
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested = true)]     // <---
    db: DbConfig,

    #[envconfig(from = "HOSTNAME")]
    hostname: String,
}
```


### Custom types

Under the hood envconfig relies on [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait.
If you want to use a custom type as a field for config, you have to implement `FromStr` trait for your custom type.

Let's say we want to extend `DbConfig` with `driver` field, which is `DbDriver` enum that represents either `Postgresql` or `Mysql`:

```rust
pub enum DbDriver {
    Postgresql,
    Mysql,
}

impl std::str::FromStr for DbDriver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "postgres" => Ok(DbDriver::Postgresql),
            "mysql" => Ok(DbDriver::Mysql),
            _ => Err(format!("Unknown DB driver: {s}"))
        }
    }
}

#[derive(Envconfig)]
pub struct DbConfig {
    // ...
    #[envconfig(from = "DB_DRIVER")]
    pub driver: DbDriver,
}
```

If this seems too cumbersome, consider using other crates like [strum](https://docs.rs/strum/latest/strum/) to derive `FromStr` automatically.

```rust
use strum::EnumString;

#[derive(EnumString)]
pub enum DbDriver {
    Postgresql,
    Mysql,
}
```

## Testing

When writing tests you should avoid using environment variables. Cargo runs Rust tests in parallel by default which means
you can end up with race conditions in your tests if two or more are fighting over an environment variable.

To solve this you can initialise your `struct` from a `HashMap<String, String>` in your tests. The `HashMap` should
match what you expect the real environment variables to be; for example `DB_HOST` environment variable becomes a
`DB_HOST` key in your `HashMap`.

```rust
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,
}

#[test]
fn test_config_can_be_loaded_from_hashmap() {
    // Create a HashMap that looks like your environment
    let mut hashmap = HashMap::new();
    hashmap.insert("DB_HOST".to_string(), "127.0.0.1".to_string());

    // Initialize config from a HashMap to avoid test race conditions
    let config = Config::init_from_hashmap(&hashmap).unwrap();

    assert_eq!(config.db_host, "127.0.0.1");
    assert_eq!(config.db_port, 5432);
}
```

## Contributing

### Running tests

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
