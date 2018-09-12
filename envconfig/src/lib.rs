#[macro_use]
extern crate failure;

mod error;
mod utils;

pub use error::Error;
pub use utils::load_var;
