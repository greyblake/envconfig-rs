# Envconfig

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

// Build Config struct or exit with a meaninful error message.
let config = Config::init_or_die();

// You can also use `init`, wich returns Result<T, Error>
let config = Config::init().unwrap();
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
