#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "HOST")]
    pub host: String,
}

// Ensure custom Result can be defined in the current context.
// See: https://github.com/greyblake/envconfig-rs/issues/21
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let res: Result<i32> = Ok(123);
    println!("{:?}", res);
}
