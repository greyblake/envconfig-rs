#[macro_use]
extern crate failure;

mod error;
mod traits;
mod utils;

pub use error::Error;
pub use utils::load_var;
pub use traits::Envconfig;
