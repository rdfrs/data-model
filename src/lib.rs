extern crate core;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0} is not a valid absolute URI")]
    InvalidURIError(String),
}
type Result<T> = std::result::Result<T, Error>;

pub mod dataset;
mod quad_enums;
mod quad_generics;
mod quad_trait_obj;
pub mod terms;
