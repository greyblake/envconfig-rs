#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "HOST")]
    pub host: u16,
}

fn main() {
}
