#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "Env variable is missing: {}", name)]
    EnvVarMissing { name: &'static str },
    #[fail(display = "Failed to parse env variable: {}", name)]
    ParseError { name: &'static str },
}
