#[macro_use]
extern crate failure;

mod error;
mod traits;
mod utils;

pub use error::Error;
pub use traits::Envconfig;
pub use utils::load_var;
